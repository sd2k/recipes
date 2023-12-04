use dioxus_fullstack::prelude::*;
use recipe_app::app;
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

// use recipe_db::create_pool;

pub fn setup_tracing() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let error_layer = ErrorLayer::default();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .with(error_layer)
        .init();
}

#[tokio::main]
async fn main() {
    setup_tracing();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(recipe_server::router().into_make_service())
        .await;
}
