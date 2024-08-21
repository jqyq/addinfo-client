use std::fmt::Display;

use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;
use serde::Serialize;

// We need an error type that we can serialize into an API response.
// We convert any internal errors to the HTTP 500 status code.
#[derive(Debug, Serialize)]
pub struct AppError {
    error: String,
}

impl<E> From<E> for AppError
where
    E: Display,
{
    fn from(e: E) -> Self {
        Self {
            error: e.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Html(self.error)).into_response()
    }
}
