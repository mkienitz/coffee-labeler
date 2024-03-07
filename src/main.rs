use axum::{
    routing::{get, post},
    Router,
};
use coffee_labeler::{api, pages};
use color_eyre::eyre::Result;
use maud::Markup;

async fn index() -> Markup {
    pages::index()
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let app = Router::new()
        .route("/", get(index))
        .route("/api/og/label", post(api::create_label_image))
        .route("/api/submit_label_form", post(api::submit_label_form))
        .route("/api/submit_bq_url", post(api::submit_bq_url));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3333").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
