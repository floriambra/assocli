use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, Serializer};

pub type ResultAx<T> = std::result::Result<T, ErrorAx>;

fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

#[derive(Debug, Serialize)]
pub struct ErrorAx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub title: String,
    #[serde(serialize_with = "serialize_status_code")]
    pub status: StatusCode,
    pub detail: String,
}

impl ErrorAx {
    pub fn new(status: StatusCode, title: impl Into<String>, detail: impl Into<String>) -> Self {
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
        Self::new(StatusCode::BAD_REQUEST, "Bad Request", detail)
    }

    pub fn unauthorized(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "Unauthorized", detail)
    }

    pub fn forbidden(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "Forbidden", detail)
    }

    pub fn not_found(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "Not Found", detail)
    }

    pub fn conflict(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "Conflict", detail)
    }

    pub fn unprocessable(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Unprocessable Entity",
            detail,
        )
    }

    // Errores 5xx - Server errors
    pub fn internal(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            detail,
        )
    }

    pub fn service_unavailable(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "Service Unavailable",
            detail,
        )
    }
}

impl IntoResponse for ErrorAx {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

impl std::fmt::Display for ErrorAx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): {}", self.title, self.status, self.detail)?;
        if let Some(t) = &self.r#type {
            write!(f, " [{}]", t)?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorAx {}
