use std::net::SocketAddr;

use axum::Server;
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

use recipe_db::create_pool;
use recipe_graphql::create_schema;
use recipe_server::create_router_with_state;

pub fn setup_tracing() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let error_layer = ErrorLayer::default();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(error_layer)
        .with(tracing_logfmt::layer())
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() {
    setup_tracing();

    let pool = create_pool().expect("could not create database pool");
    let schema = create_schema(pool.clone());
    let app = create_router_with_state(schema);

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
