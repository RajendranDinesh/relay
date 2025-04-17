use axum::{
    routing::{get, post},
    Router,
    serve,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod auth;

use config::{AppConfig, create_db_pool};
use errors::AppError;

// Shared application state
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    config: AppConfig,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing (logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting application");

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("Configuration loaded successfully");

    // Create database connection pool
    let db_pool = create_db_pool(&config.database_url).await?;
    info!("Database pool created successfully");

    // Run migrations
    // info!("Running database migrations...");
    // sqlx::migrate!("./migrations") // Point to your migrations folder
    //     .run(&db_pool)
    //     .await
    //     .map_err(|e| AppError::DatabaseError(sqlx::Error::Migrate(Box::new(e))))?;
    // info!("Migrations completed.");

    // Create application state
    let app_state = AppState {
        db_pool,
        config, // Clone config into state
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any) // Adjust for production (e.g., specific origins)
        .allow_methods(Any) // Or specify methods like GET, POST, PUT, DELETE
        .allow_headers(Any); // Or specify headers like Content-Type, Authorization

    // Build application routes
    let app = Router::new()
        .route("/health", get(|| async { "OK" })) // Simple health check
        .route("/register", post(handlers::auth::register_handler))
        .route("/login", post(handlers::auth::login_handler))
        .route("/device", post(handlers::device::register_device))
        .route("/device", get(handlers::device::find_all_user_devices))
        .route("/sms", post(handlers::sms::sms_handler))
        // Apply state and CORS layer
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());


    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // Listen on all interfaces, port 3000
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| AppError::InternalServerError(format!("Failed to bind address: {}", e)))?; // Handle bind error

    serve(listener, app.into_make_service())
        .await
        .map_err(|e| AppError::InternalServerError(format!("Server error: {}", e)))?; // Handle server error

    Ok(())
}
