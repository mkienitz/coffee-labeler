use std::str::FromStr;

use axum::{http::header, response::IntoResponse, Form, Json};
use base64::{prelude::BASE64_STANDARD, Engine};
use protobuf::Message;
use serde::Deserialize;

use crate::{components, error::AppError, protos::bean, types::BeanInfo};
use color_eyre::eyre::{eyre, OptionExt, Result};

use maud::Markup;

mod render;
use render::render_markup;

pub async fn create_label_image(
    Json(bean_info): Json<BeanInfo>,
) -> Result<impl IntoResponse, AppError> {
    Ok((
        [(header::CONTENT_TYPE, "image/png")],
        render_markup(components::label(&bean_info)).await?,
    )
        .into_response())
}

pub async fn submit_label_form(Form(bean_info): Form<BeanInfo>) -> Result<Markup, AppError> {
    Ok(components::label(&bean_info))
}

#[derive(Deserialize)]
pub struct BqForm {
    url: String,
    dose_weight: f32,
}

pub async fn submit_bq_url(Form(form_data): Form<BqForm>) -> Result<Markup, AppError> {
    let url = form_data.url;
    let stripped: String = url
        .strip_prefix("https://beanconqueror.com?")
        .ok_or_eyre("Malformed URL")?
        .split('&')
        .filter_map(|p| p.split_once('='))
        .map(|a| a.1)
        .collect();
    let bytes = BASE64_STANDARD.decode(stripped)?;
    let bean = bean::BeanProto::parse_from_bytes(&bytes)?;
    let info = bean
        .bean_information
        .first()
        .ok_or_eyre("No bean information found!")?;

    let country_alpha2 = info
        .country
        .clone()
        .ok_or("Couldn't find country!")
        .and_then(|c| celes::Country::from_str(&c))
        .map_err(|e| eyre!(e))?
        .alpha2
        .to_owned();

    let bean_info = BeanInfo {
        country: country_alpha2,
        name: bean.name,
        roaster: bean.roaster.ok_or_eyre("Couldn't find roaster!")?,
        varietals: info.variety.clone().ok_or_eyre("Couldn't find roaster!")?,
        region: info.region.as_ref().unwrap_or(&"-".to_owned()).to_owned(),
        farm: info.farm.clone().ok_or_eyre("Couldn't find farm!")?,
        elevation: info
            .elevation
            .as_ref()
            .unwrap_or(&"-".to_owned())
            .to_owned(),
        dose_weight: format!("{}g", form_data.dose_weight.to_string()),
        roasting_date: bean
            .roastingDate
            .ok_or_eyre("Couldn't find roasting date!")?[2..10]
            .into(),
        processing: info
            .processing
            .clone()
            .ok_or_eyre("Couldn't find processing!")?,
        aromatics: bean.aromatics.ok_or_eyre("Couldn't find Aromatics!")?,
    };
    Ok(components::label(&bean_info))
}
