use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub title: String,
    pub status: u16,
    pub detail: String,
}

impl Error {
    pub fn new(status: u16, title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            r#type: None,
            title: title.into(),
            status,
            detail: detail.into(),
        }
    }

    pub fn with_type(mut self, error_type: impl Into<String>) -> Self {
        self.r#type = Some(error_type.into());
        self
    }

    // Errores 4xx - Client errors
    pub fn bad_request(detail: impl Into<String>) -> Self {
        Self::new(400, "Bad Request", detail)
    }

    pub fn unauthorized(detail: impl Into<String>) -> Self {
        Self::new(401, "Unauthorized", detail)
    }

    pub fn forbidden(detail: impl Into<String>) -> Self {
        Self::new(403, "Forbidden", detail)
    }

    pub fn not_found(detail: impl Into<String>) -> Self {
        Self::new(404, "Not Found", detail)
    }

    pub fn conflict(detail: impl Into<String>) -> Self {
        Self::new(409, "Conflict", detail)
    }

    pub fn unprocessable(detail: impl Into<String>) -> Self {
        Self::new(422, "Unprocessable Entity", detail)
    }

    // Errores 5xx - Server errors
    pub fn internal(detail: impl Into<String>) -> Self {
        Self::new(500, "Internal Server Error", detail)
    }

    pub fn service_unavailable(detail: impl Into<String>) -> Self {
        Self::new(503, "Service Unavailable", detail)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): {}", self.title, self.status, self.detail)?;
        if let Some(t) = &self.r#type {
            write!(f, " [{}]", t)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
