use recipe_db::DbPool;
use recipe_graphql::create_schema;
use recipe_server::{create_router, AppState};

mod pg_pool;
mod service_info;

#[shuttle_runtime::main]
async fn axum(
    #[pg_pool::ShuttleDbPool] pool: DbPool,
    #[shuttle_metadata::ShuttleMetadata] metadata: shuttle_metadata::Metadata,
) -> shuttle_axum::ShuttleAxum {
    let schema = create_schema(pool);
    let api_url = Box::leak(Box::new(format!(
        "https://{}.shuttleapp.rs/graphql",
        metadata.service_name()
    )));
    let router = create_router().with_state(AppState::new(schema).with_api_url(api_url.as_str()));

    Ok(router.into())
}
