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
        .with(filter_layer)
        .with(error_layer)
        .init();
}

fn main() {
    setup_tracing();

    let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../recipe-web/dist");
    let cfg = ServeConfigBuilder::new(app, ()).assets_path(assets_path);
    LaunchBuilder::new(app).server_cfg(cfg).launch()
}
