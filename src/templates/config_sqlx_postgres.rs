pub mod sqlx {
    use sqlx::{PgPool, postgres::PgPoolOptions};
    use std::time::Duration;

    pub async fn connection(url: &str) -> PgPool {
        let connection_manager = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .test_before_acquire(true)
            .connect(url)
            .await;

        match connection_manager {
            Ok(pool) => {
                tracing::info!("Successful database connection pool sqlx postgres");
                if let Err(e) = sqlx::query("SELECT 1").execute(&pool).await {
                    tracing::error!("Database connectivity test failed: {:?}", e);
                    std::process::exit(1);
                }

                pool
            }
            Err(err) => {
                tracing::error!("Error creating database pool sqlx postgres: {:?}", err);
                std::process::exit(1)
            }
        }
    }
}
