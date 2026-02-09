use deeppdcfr_mock_server::{create_app, get_config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "deeppdcfr_mock_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration
    let config = get_config();
    let addr = config.addr;

    // Create app
    let app = create_app();

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("🚀 Server starting on http://{}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/docs", addr);

    // Run server
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
