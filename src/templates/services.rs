pub(crate) mod service {

    use super::super::{
        models::model::dto::{CreateUpdateGENERIC, GENERIC},
        repositories::repository::Repository,
    };
    use crate::app::shared::common::error::{ErrorAx, ResultAx};

    pub(crate) struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub(crate) async fn get(&self) -> ResultAx<Vec<GENERIC>> {
            // add code here
            match self.repository.get_all() {
                Ok(items) => {
                    if !items.is_empty() {
                        Ok(items)
                    } else {
                        Err(ErrorAx::not_found("There are no articles"))
                    }
                }
                Err(err) => {
                    tracing::error!(err);
                    Err(ErrorAx::service_unavailable(" "))
                }
            }
        }

        pub(crate) async fn get_by_id(&self, _id: i32) -> ResultAx<Option<GENERIC>> {
            match self.repository.get_by_id(_id) {
                Ok(item) => {
                    if let Some(value) = item {
                        Ok(Some(value))
                    } else {
                        Err(ErrorAx::not_found("item not found"))
                    }
                }
                Err(err) => {
                    tracing::error!(err);
                    Err(ErrorAx::service_unavailable(""))
                }
            }
        }

        pub(crate) async fn create(&self, _dto: CreateUpdateGENERIC) -> ResultAx<GENERIC> {
            let mut new_item = GENERIC::default();

            if let Some(id) = _dto.id {
                new_item.id = id
            } else {
                return Err(ErrorAx::bad_request("id cannot be null"));
            };

            if let Some(name) = _dto.name {
                new_item.name = name;
            } else {
                return Err(ErrorAx::bad_request("name cannot be null"));
            };

            if let Some(state) = _dto.state {
                new_item.state = state;
            } else {
                return Err(ErrorAx::bad_request("state cannot be null"));
            };

            match self.repository.create(new_item) {
                Ok(item) => {
                    if let Some(value) = item {
                        Ok(value)
                    } else {
                        Err(ErrorAx::conflict("The item already exists"))
                    }
                }
                Err(err) => {
                    tracing::error!(err);
                    Err(ErrorAx::service_unavailable(""))
                }
            }
        }

        pub(crate) async fn update(
            &self,
            _dto: CreateUpdateGENERIC,
            _id: i32,
        ) -> ResultAx<(i32, GENERIC)> {
            let mut modified_item = GENERIC::default();

            if _dto.id.is_none() {
                return Err(ErrorAx::bad_request("id cannot be modified"));
            }

            if let Some(name) = _dto.name {
                modified_item.name = name;
            } else {
                modified_item.name = "".to_string();
            };

            if let Some(state) = _dto.state {
                modified_item.state = state;
            } else {
                modified_item.state = "".to_string();
            };
            match self.repository.update(_id, modified_item) {
                Ok(item) => {
                    if let Some(value) = item.1 {
                        Ok((item.0, value))
                    } else {
                        Err(ErrorAx::not_found("The item does not exist"))
                    }
                }
                Err(err) => {
                    tracing::error!(err);
                    Err(ErrorAx::service_unavailable(""))
                }
            }
        }

        pub(crate) async fn delete(&self, _id: i32) -> ResultAx<()> {
            match self.repository.delete(_id) {
                Ok(status) => {
                    if status {
                        Ok(())
                    } else {
                        Err(ErrorAx::conflict("item"))
                    }
                }
                Err(err) => {
                    tracing::error!(err);
                    Err(ErrorAx::service_unavailable(""))
                }
            }
        }
    }
}
