use axum::Router;
use dioxus_fullstack::{axum_adapter::DioxusRouterExt, prelude::*};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use recipe_app::app;

pub fn router() -> Router {
    let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../recipe-web/dist");
    let cfg = ServeConfigBuilder::new(app, ())
        .assets_path(assets_path)
        .build();
    Router::new().serve_dioxus_application("", cfg).layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new().gzip(true))
            .layer(TraceLayer::new_for_http()),
    )
}
