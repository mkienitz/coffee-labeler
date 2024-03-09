use std::env;

use axum::{
    routing::{get, post},
    Router,
};

use coffee_labeler::{api, config::AppState, pages};
use color_eyre::eyre::Result;

use maud::Markup;

async fn index() -> Markup {
    pages::index()
}

#[tokio::main]
async fn main() -> Result<()> {
    let port = env::var("COFFEE_LABELER_PORT")?;
    let host = env::var("COFFEE_LABELER_HOST")?;
    let printer_host = env::var("COFFEE_LABELER_PRINTER_HOST")?;
    let printer_port = env::var("COFFEE_LABELER_PRINTER_PORT")?;

    let state = AppState {
        printer_address: format!("{printer_host}:{printer_port}"),
    };

    color_eyre::install()?;
    let app = Router::new()
        .route("/", get(index))
        .route("/api/og/label", post(api::create_label_image))
        .route("/api/update_label", post(api::update_label))
        .route("/api/print_label", post(api::print_label))
        .route("/api/load_from_bq", post(api::load_from_bq))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
