use crate::app::shared::common::error::ErrorAx;
use axum::{
    Json,
    body::Body,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = ErrorAx;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // Usa el extractor Json de Axum que ya sabe manejar el estado
        let Json(value) =
            Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection: JsonRejection| {
                    ErrorAx::bad_request(format!("Invalid JSON: {}", rejection))
                })?;

        // Valida
        value.validate().map_err(|validation_errors| {
            let errors: Vec<String> = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        let message = error
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("{} validation failed", field));
                        format!("{}: {}", field, message)
                    })
                })
                .collect();
            ErrorAx::unprocessable(errors.join(", "))
        })?;

        Ok(ValidatedJson(value))
    }
}
