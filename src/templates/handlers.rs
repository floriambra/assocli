pub(crate) mod handler {
    use super::super::{models::model::dto::CreateUpdateGENERIC, services::service::Service};
    use crate::app::shared::common::{error::ResultAx, validation::ValidatedJson};
    use axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
    };

    pub async fn get_all(
        service: State<std::sync::Arc<Service>>,
    ) -> ResultAx<Json<serde_json::Value>> {
        let items = service.get().await?;
        Ok(Json(serde_json::json!({"items" : items})))
    }

    pub async fn get_by_id(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
    ) -> ResultAx<Json<serde_json::Value>> {
        let items = service.get_by_id(_id).await?;
        Ok(Json(serde_json::json!({"items" : items})))
    }

    pub async fn create(
        service: State<std::sync::Arc<Service>>,
        ValidatedJson(request): ValidatedJson<CreateUpdateGENERIC>,
    ) -> ResultAx<(StatusCode, Json<serde_json::Value>)> {
        let item = service.create(request).await?;
        Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({"item" : item})),
        ))
    }

    pub async fn update(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
        ValidatedJson(request): ValidatedJson<CreateUpdateGENERIC>,
    ) -> ResultAx<Json<serde_json::Value>> {
        let item = service.update(request, _id).await?;
        Ok(Json(serde_json::json!({
            "item": item.1
        })))
    }

    pub async fn delete(
        Path(_id): Path<i32>,
        service: State<std::sync::Arc<Service>>,
    ) -> ResultAx<StatusCode> {
        service.delete(_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
