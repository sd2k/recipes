use recipe_app::server::AppState;
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
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    let pool = recipe_db::create_pool().expect("create db pool");
    let repo = recipe_repository::DieselRepository::new(pool);
    let state = AppState::new(repo);
    axum::Server::bind(&addr)
        .serve(recipe_server::router(state).into_make_service())
        .await
        .unwrap()
}
