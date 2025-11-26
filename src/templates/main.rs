mod app;
use crate::app::config::env::env::Env;
use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let var_env = Env::init();
    let env_port: u16 = var_env.get_parsed("PORT").unwrap_or_else(|| 3000);
    let env_address: &str = &var_env.get_or("ADDRESS", "127.0.0.1");
    let app = Router::new();

    let listener = TcpListener::bind(format!("{env_address}:{env_port}"))
        .await
        .unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true),
        )
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .init();

    tracing::info!("󰒋 Server starting on {}:{}", env_address, env_port);
    axum::serve(listener, app).await.unwrap();
}

