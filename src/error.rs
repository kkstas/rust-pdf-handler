use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    AsdfError,
    ServerError,
}
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - error response", "ERROR");
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
