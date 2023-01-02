use std::path::PathBuf;

use sync_wrapper::SyncWrapper;

use recipe_db::DbPool;
use recipe_graphql::create_schema;
use recipe_server::{create_servedir_router, AppState};

mod pg_pool;

#[shuttle_service::main]
async fn axum(
    #[pg_pool::ShuttleDbPool] pool: DbPool,
    #[shuttle_static_folder::StaticFolder(folder = "dist")] assets_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let schema = create_schema(pool);
    let router = create_servedir_router(&assets_folder).with_state(AppState::new(schema));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
