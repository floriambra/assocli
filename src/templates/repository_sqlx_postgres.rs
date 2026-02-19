pub(crate) mod repository {

    use super::super::models::model::dto::GENERIC;
    use sqlx::{Error as error_sqlx, PgPool};

    pub struct Repository {
        pool: PgPool,
    }

    impl Repository {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }

        pub fn get_all(&self) -> Result<Vec<GENERIC>, error_sqlx> {
            let items: Vec<Team> = vec![];
            Ok(items)
        }

        pub fn get_by_id(&self, _id: i32) -> Result<Vec<GENERIC>, error_sqlx> {
            let items: Vec<Team> = vec![];
            Ok(items)
        }

        pub fn create(&self, _dto: Team) -> Result<GENERIC, error_sqlx> {
            Ok(_dto)
        }

        pub fn update(&self, _id: i32, _dto: Team) -> Result<(i32, GENERIC), error_sqlx> {
            Ok((_id, _dto))
        }

        pub fn delete(&self, _id: i32) -> Result<bool, error_sqlx> {
            Ok(false)
        }
    }
}
