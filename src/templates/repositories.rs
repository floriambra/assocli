pub(crate) mod repository {

    use super::super::models::model::dto::GENERIC;
    pub struct Repository {
        items: std::sync::Arc<std::sync::Mutex<Vec<GENERIC>>>,
    }

    impl Repository {
        pub fn new() -> Self {
            let initial_data = vec![
                GENERIC {
                    id: 1,
                    name: "Eduard Golfinc".to_string(),
                    state: "on".to_string(),
                },
                GENERIC {
                    id: 2,
                    name: "Emma Rulphin".to_string(),
                    state: "on".to_string(),
                },
                GENERIC {
                    id: 3,
                    name: "Wilhelm Renfort".to_string(),
                    state: "false".to_string(),
                },
            ];

            Self {
                items: std::sync::Arc::new(std::sync::Mutex::new(initial_data)),
            }
        }

        pub fn get_all(&self) -> Result<Vec<GENERIC>, String> {
            let items = self.items.lock().map_err(|e| e.to_string())?;
            Ok(items.clone())
        }

        pub fn get_by_id(&self, _id: i32) -> Result<Option<GENERIC>, String> {
            let items = self.items.lock().map_err(|e| e.to_string())?;
            Ok(items.iter().find(|item| item.id == _id).cloned())
        }

        pub fn create(&self, _dto: GENERIC) -> Result<Option<GENERIC>, String> {
            let mut items = self.items.lock().map_err(|e| e.to_string())?;

            if items.iter().any(|b| b.id == _dto.id) {
                return Ok(None::<GENERIC>);
            }

            items.push(_dto.clone());
            Ok(Some(_dto))
        }

        pub fn update(&self, _id: i32, _dto: GENERIC) -> Result<(i32, Option<GENERIC>), String> {
            let mut items = self.items.lock().map_err(|e| e.to_string())?;

            if let Some(value) = items.iter_mut().find(|b| b.id == _id) {
                if !_dto.name.is_empty() {
                    value.name = _dto.name;
                };

                if !_dto.state.is_empty() {
                    value.state = _dto.state;
                };

                Ok((_id, Some(value.clone())))
            } else {
                Ok((_id, None::<GENERIC>))
            }
        }

        pub fn delete(&self, _id: i32) -> Result<bool, String> {
            let mut items = self.items.lock().map_err(|e| e.to_string())?;
            let len_before = items.len();
            items.retain(|value| value.id != _id);

            Ok(items.len() < len_before)
        }
    }
}
