#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        tracing::info!("🔧 Initializing AppState...");
        Self {}
    }
}
