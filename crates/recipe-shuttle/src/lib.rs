use sync_wrapper::SyncWrapper;

use recipe_db::DbPool;
use recipe_graphql::create_schema;
use recipe_server::{create_router, AppState};

mod pg_pool;

#[shuttle_service::main]
async fn axum(
    #[pg_pool::ShuttleDbPool] pool: DbPool,
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_service::ShuttleAxum {
    let schema = create_schema(pool);
    let api_url = Box::leak(Box::new(
        secret_store
            .get("API_URL")
            .expect("API_URL must be set in Secrets.toml"),
    ));
    let router = create_router().with_state(AppState::new(schema).with_api_url(api_url.as_str()));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
