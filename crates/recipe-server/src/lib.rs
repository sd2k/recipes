use std::{future::ready, path::Path};

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    body::{boxed, Full},
    extract::State,
    headers::{self, Header},
    http::{header, StatusCode, Uri},
    response::{self, Html, IntoResponse, Response},
    routing::{get, get_service},
    Router, TypedHeader,
};
use dioxus::prelude::*;
#[cfg(not(feature = "embed"))]
use rust_embed::RustEmbed;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, ServiceBuilderExt};

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
    api_url: &'static str,
}

impl AppState {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            api_url: "http://localhost:8000/graphql",
        }
    }

    pub fn with_api_url(mut self, api_url: &'static str) -> Self {
        self.api_url = api_url;
        self
    }
}

async fn graphql_handler(state: State<AppState>, req: GraphQLRequest) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphiql").finish())
}

#[derive(Debug, Clone, Copy, Default)]
struct AcceptEncoding {
    gzip: bool,
    brotli: bool,
    deflate: bool,
}

impl Header for AcceptEncoding {
    fn name() -> &'static headers::HeaderName {
        &header::ACCEPT_ENCODING
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i header::HeaderValue>,
    {
        values
            .next()
            .map(|value| {
                let mut gzip = false;
                let mut brotli = false;
                let mut deflate = false;
                for part in value
                    .to_str()
                    .map_err(|_| headers::Error::invalid())?
                    .split(',')
                {
                    match part.trim() {
                        "gzip" => gzip = true,
                        "br" => brotli = true,
                        "deflate" => deflate = true,
                        _ => {}
                    }
                }
                Ok(AcceptEncoding {
                    gzip,
                    brotli,
                    deflate,
                })
            })
            .unwrap_or_else(|| Ok(AcceptEncoding::default()))
    }

    fn encode<E: Extend<header::HeaderValue>>(&self, values: &mut E) {
        if self.gzip {
            values.extend(Some(header::HeaderValue::from_static("gzip")));
        }
        if self.brotli {
            values.extend(Some(header::HeaderValue::from_static("br")));
        }
        if self.deflate {
            values.extend(Some(header::HeaderValue::from_static("deflate")));
        }
    }
}

async fn static_handler(
    uri: Uri,
    TypedHeader(accept_encoding): TypedHeader<AcceptEncoding>,
    state: State<AppState>,
) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() {
        return index_html(state).await;
    }

    let asset = if accept_encoding.brotli {
        Assets::get(&format!("{}.br", path))
            .or_else(|| Assets::get(path))
            .map(|asset| asset.data)
    } else {
        Assets::get(path).map(|asset| asset.data)
    };

    match asset {
        Some(data) => {
            let body = boxed(Full::from(data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => index_html(state).await,
    }
}

async fn index_html(state: State<AppState>) -> Response {
    // TODO: ideally we would just do a database call and construct the
    // types manually here rather than going through the API.
    let req = AllRecipes::build(()).try_into().unwrap();

    let initial_state = reqwest::Client::new()
        .post(state.api_url)
        .run_graphql(req)
        .await
        .unwrap()
        .data
        .into();
    let mut app = VirtualDom::new_with_props(
        recipe_app::app,
        recipe_app::RootProps {
            origin: state.api_url.clone(),
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

pub fn create_servedir_router(path: &Path) -> Router<AppState> {
    Router::new()
        .route("/", get(index_html))
        .route("/graphql", get(graphiql).post(graphql_handler))
        .nest_service(
            "/assets",
            get_service(ServeDir::new(&path)).handle_error(|_| ready(StatusCode::NOT_FOUND)),
        )
        .layer(ServiceBuilder::new().trace_for_http())
}

pub fn create_router_with_state(schema: Schema) -> Router {
    create_router().with_state(AppState::new(schema))
}
