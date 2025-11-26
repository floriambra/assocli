pub(crate) mod service {

    use super::super::{models::model::dto::GENERIC, repositories::repository::Repository};
    use crate::app::shared::common::error::{Error, Result};

    pub(crate) struct Service {
        repository: Repository,
    }

    impl Service {
        pub fn new(repository: Repository) -> Self {
            Self { repository }
        }

        pub(crate) async fn get(&self) -> Result<Vec<GENERIC>> {
            // add code here
            match self.repository.get_all() {
                Ok(items) => {
                    if !items.is_empty() {
                        Ok(items)
                    } else {
                        Err(Error::not_found("There are no articles"))
                    }
                }
                Err(_) => Err(Error::service_unavailable(" ")),
            }
        }

        pub(crate) async fn get_by_id(&self, _id: i32) -> Result<Vec<GENERIC>> {
            match self.repository.get_by_id(_id) {
                Ok(items) => {
                    let item_size = items.len();
                    if item_size > 0 {
                        Ok(items)
                    } else {
                        Err(Error::not_found(format!(
                            "There is no article with the id {_id}"
                        )))
                    }
                }
                Err(_) => Err(Error::service_unavailable("")),
            }
        }

        pub(crate) async fn create(&self, _dto: GENERIC) -> Result<GENERIC> {
            match self.repository.create(_dto) {
                Ok(item) => Ok(item),
                Err(_) => Err(Error::service_unavailable("")),
            }
        }

        pub(crate) async fn update(&self, _dto: GENERIC, _id: i32) -> Result<(i32, GENERIC)> {
            match self.repository.update(_id, _dto) {
                Ok(item) => Ok(item),
                Err(_) => Err(Error::service_unavailable("")),
            }
        }

        pub(crate) async fn delete(&self, _id: i32) -> Result<()> {
            match self.repository.delete(_id) {
                Ok(status) => {
                    if status {
                        Ok(())
                    } else {
                        Err(Error::conflict("item"))
                    }
                }
                Err(_) => Err(Error::service_unavailable("")),
            }
        }
    }
}
