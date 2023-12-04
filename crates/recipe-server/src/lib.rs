use axum::{
    body::Body,
    http::{Response, StatusCode},
    routing::get,
    Router,
};
use dioxus_fullstack::{axum_adapter::DioxusRouterExt, prelude::*, server_fn_service};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use recipe_app::{app, server::AppState};
use tracing::debug;

pub fn router(state: AppState) -> Router {
    recipe_app::server::register_explicit();

    let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../recipe-web/dist");
    let cfg = ServeConfigBuilder::new(app, ())
        .assets_path(assets_path)
        .build();

    let ssr_state = SSRState::new(&cfg);
    Router::new()
        .serve_static_assets(assets_path)
        .connect_hot_reload()
        .register_server_fns_with_handler("", |func| {
            let state = state.clone();
            move |req| async move {
                debug!("injecting state into server fn");

                let mut server_context = DioxusServerContext::default();
                if server_context.insert(state).is_err() {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Error injecting state"))
                        .expect("create error response");
                }
                let mut service = server_fn_service(server_context, func);
                match service.run(req).await {
                    Ok(res) => res,
                    Err(e) => {
                        let mut res = Response::new(Body::from(e.to_string()));
                        *res.status_mut() = match e {
                            ServerFnError::Request(_) => StatusCode::BAD_GATEWAY,
                            _ => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        res
                    }
                }
            }
        })
        .fallback(get(render_handler_with_state).with_state((state.clone(), cfg, ssr_state)))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new().gzip(true))
                .layer(TraceLayer::new_for_http()),
        )
}
