use std::io::Cursor;
use std::str::FromStr;

use axum::{extract::State, http::header, response::IntoResponse, Form, Json};
use base64::{prelude::BASE64_STANDARD, Engine};

use protobuf::Message;
use serde::Deserialize;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{components, config::AppState, error::AppError, protos::bean, types::BeanInfo};
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

pub async fn update_label(Json(bean_info): Json<BeanInfo>) -> Result<Markup, AppError> {
    Ok(components::label(&bean_info))
}

#[derive(Deserialize)]
pub struct PrintInfo {
    bean_info: BeanInfo,
    no_pages: u8,
}

pub async fn print_label(
    State(state): State<AppState>,
    Json(print_info): Json<PrintInfo>,
) -> Result<Markup, AppError> {
    let screenshot = render_markup(components::label(&print_info.bean_info)).await?;
    let reader = image::io::Reader::new(Cursor::new(screenshot)).with_guessed_format()?;
    let print_job = brother_ql::printjob::PrintJob {
        no_pages: print_info.no_pages,
        image: reader.decode()?,
        media: brother_ql::media::Media::C62,
        high_dpi: false,
        compressed: false,
        quality_priority: true,
        cut_behaviour: brother_ql::printjob::CutBehavior::CutEach,
    }
    .compile()?;
    let mut stream = TcpStream::connect(state.printer_address.clone()).await?;
    let _bytes_written = stream.write(&print_job).await?;
    Ok(components::label(&print_info.bean_info))
}

#[derive(Deserialize)]
pub struct BqForm {
    url: String,
    dose_weight: f32,
}

pub async fn load_from_bq(Form(form_data): Form<BqForm>) -> Result<Markup, AppError> {
    let proto_string = url::Url::parse(&form_data.url)?
        .query_pairs()
        .map(|v| v.1)
        .collect::<String>()
        .replace(' ', "+");
    let bean_proto = BASE64_STANDARD
        .decode(proto_string)
        .map(|s| bean::BeanProto::parse_from_bytes(&s))??;
    let bean_proto_info = bean_proto
        .bean_information
        .first()
        .ok_or_eyre("No bean information found!")?;
    let country_alpha2 = bean_proto_info
        .country
        .as_ref()
        .ok_or("Couldn't find country!")
        .and_then(|c| celes::Country::from_str(&c.replace(' ', "_")))
        .map_err(|e| eyre!(e))?
        .alpha2
        .to_owned();
    let bean_info = BeanInfo {
        country: country_alpha2,
        name: bean_proto.name,
        roaster: bean_proto.roaster.ok_or_eyre("Couldn't find roaster!")?,
        varietals: bean_proto_info
            .variety
            .clone()
            .ok_or_eyre("Couldn't find varietals!")?,
        region: bean_proto_info.region.as_deref().unwrap_or("-").to_owned(),
        farm: bean_proto_info
            .farm
            .clone()
            .ok_or_eyre("Couldn't find farm!")?,
        elevation: bean_proto_info
            .elevation
            .as_deref()
            .unwrap_or("-")
            .to_owned(),
        dose_weight: format!("{}g", form_data.dose_weight),
        roasting_date: bean_proto
            .roastingDate
            .ok_or_eyre("Couldn't find roasting date!")?[2..10]
            .into(),
        processing: bean_proto_info
            .processing
            .clone()
            .ok_or_eyre("Couldn't find processing!")?,
        aromatics: bean_proto
            .aromatics
            .ok_or_eyre("Couldn't find aromatics!")?,
    };
    Ok(components::label(&bean_info))
}
