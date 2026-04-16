pub(crate) mod repository {
    use super::super::models::model::dto::GENERIC;
    use sqlx::{Error as error_sqlx, MySqlPool};

    pub struct Repository {
        pool: MySqlPool,
    }

    impl Repository {
        pub fn new(pool: MySqlPool) -> Self {
            Self { pool }
        }

        pub fn get_all(&self) -> Result<Vec<GENERIC>, error_sqlx> {
            let _ = &self.pool;
            let items: Vec<GENERIC> = vec![];
            Ok(items)
        }

        pub fn get_by_id(&self, _id: i32) -> Result<Option<GENERIC>, error_sqlx> {
            let _ = &self.pool;
            Ok(None)
        }

        pub fn create(&self, _dto: GENERIC) -> Result<Option<GENERIC>, error_sqlx> {
            let _ = &self.pool;
            Ok(Some(_dto))
        }

        pub fn update(
            &self,
            _id: i32,
            _dto: GENERIC,
        ) -> Result<(i32, Option<GENERIC>), error_sqlx> {
            let _ = &self.pool;
            Ok((_id, Some(_dto)))
        }

        pub fn delete(&self, _id: i32) -> Result<bool, error_sqlx> {
            let _ = &self.pool;
            Ok(false)
        }
    }
}
