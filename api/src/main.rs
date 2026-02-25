use deeppdcfr_mock_server::{configure_app, create_cors, create_swagger, get_config};
use actix_web::{App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "deeppdcfr_mock_server=info,actix_web=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration
    let config = get_config();
    let addr = config.addr;

    tracing::info!("🚀 Server starting on http://{}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/docs/", addr);

    // Run server
    HttpServer::new(move || {
        App::new()
            .wrap(create_cors())
            .service(create_swagger())
            .configure(configure_app)
    })
    .bind(&addr)?
    .run()
    .await
}
