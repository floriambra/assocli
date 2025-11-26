pub(crate) mod handler {
    use super::super::{models::model::dto::GENERIC, services::service::Service};
    use crate::app::shared::common::{error::Result, validation::ValidatedJson};
    use axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
    };

    pub async fn get_all(
        service: State<std::sync::Arc<Service>>,
    ) -> Result<Json<serde_json::Value>> {
        let items = service.get().await?;
        Ok(Json(serde_json::json!({"items" : items})))
    }

    pub async fn get_by_id(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
    ) -> Result<Json<serde_json::Value>> {
        let items = service.get_by_id(_id).await?;
        Ok(Json(serde_json::json!({"items" : items})))
    }

    pub async fn create(
        service: State<std::sync::Arc<Service>>,
        ValidatedJson(request): ValidatedJson<GENERIC>,
    ) -> Result<(StatusCode, Json<serde_json::Value>)> {
        let item = service.create(request).await?;
        Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({"item" : item})),
        ))
    }

    pub async fn update(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
        ValidatedJson(request): ValidatedJson<GENERIC>,
    ) -> Result<Json<serde_json::Value>> {
        let item = service.update(request, _id).await?;
        Ok(Json(serde_json::json!({
            "id": item.0,
            "item": item.1
        })))
    }

    pub async fn delete(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
    ) -> Result<StatusCode> {
        service.delete(_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
