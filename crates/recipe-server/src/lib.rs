use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    body::{boxed, Full},
    debug_handler,
    extract::State,
    http::{header, Uri},
    response::{self, Html, IntoResponse, Response},
    routing::get,
    Router,
};
use dioxus::prelude::*;
#[cfg(not(feature = "embed"))]
use rust_embed::RustEmbed;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing::info;

use recipe_client::prelude::*;
use recipe_graphql::Schema;
#[cfg(feature = "embed")]
use recipe_web::Assets;

#[cfg(not(feature = "embed"))]
#[derive(Debug, RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../recipe-web/dist"]
#[prefix = "assets/"]
struct Assets;

struct Index {
    prefix: String,
    suffix: String,
}

#[cfg(not(debug_assertions))]
static INDEX: once_cell::sync::Lazy<Index> = once_cell::sync::Lazy::new(|| {
    let index_bytes = Assets::get("assets/index.html").unwrap().data;
    let index = std::str::from_utf8(&index_bytes).unwrap();
    let (prefix, suffix) = index.split_once(r#"<div id="main">"#).unwrap();
    Index {
        prefix: prefix.to_string(),
        suffix: suffix.to_string(),
    }
});
#[cfg(not(debug_assertions))]
fn get_index() -> &'static Index {
    &*INDEX
}
#[cfg(debug_assertions)]
fn get_index() -> Index {
    let index_bytes = Assets::get("assets/index.html").unwrap().data;
    let index = String::from_utf8(index_bytes.into_owned()).unwrap();
    let (prefix, suffix) = index.split_once(r#"<div id="main">"#).unwrap();
    Index {
        prefix: prefix.to_string(),
        suffix: suffix.to_string(),
    }
}

#[derive(Clone)]
pub struct AppState {
    schema: Schema,
}

impl AppState {
    pub fn new(schema: Schema) -> Self {
        Self { schema }
    }
}

async fn graphql_handler(state: State<AppState>, req: GraphQLRequest) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphiql").finish())
}

#[debug_handler]
async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() {
        return index_html().await;
    }

    info!("path: {}", path);
    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => index_html().await,
    }
}

async fn index_html() -> Response {
    // TODO: ideally we would just do a database call and construct the
    // types manually here rather than going through the API.
    let req = AllRecipes::build(()).try_into().unwrap();
    let initial_state = reqwest::Client::new()
        .post("http://localhost:8000/graphql")
        .run_graphql(req)
        .await
        .unwrap()
        .data
        .into();
    let mut app = VirtualDom::new_with_props(
        recipe_app::app,
        recipe_app::RootProps {
            origin: "http://localhost:8000",
            initial_state,
        },
    );
    let _ = app.rebuild();
    let html = dioxus::ssr::render_vdom_cfg(&app, |c| c.pre_render(true));
    let Index { prefix, suffix } = get_index();
    Html(format!(r#"{prefix}<div id="main">{html}{suffix}"#)).into_response()
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .fallback(static_handler)
        .layer(ServiceBuilder::new().trace_for_http())
}

pub fn create_router_with_state(schema: Schema) -> Router {
    create_router().with_state(AppState::new(schema))
}
