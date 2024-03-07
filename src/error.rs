use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(color_eyre::eyre::Error);

impl<E> From<E> for AppError
where
    E: Into<color_eyre::eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        println!("Internal error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error occured!",
        )
            .into_response()
    }
}
