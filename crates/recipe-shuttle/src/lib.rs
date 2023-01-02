use sync_wrapper::SyncWrapper;

use recipe_db::DbPool;
use recipe_graphql::create_schema;
use recipe_server::create_router_with_state;

mod pg_pool;

#[shuttle_service::main]
async fn axum(#[pg_pool::ShuttleDbPool] pool: DbPool) -> shuttle_service::ShuttleAxum {
    let schema = create_schema(pool);
    let router = create_router_with_state(schema);
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
