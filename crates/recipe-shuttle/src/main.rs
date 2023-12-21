use recipe_app::server::AppState;
use recipe_db::DbPool;
use recipe_repository::DieselRepository;
use recipe_server::{router, HotReload};

mod pg_pool;

#[shuttle_runtime::main]
async fn main(#[pg_pool::ShuttleDbPool] pool: DbPool) -> shuttle_axum::ShuttleAxum {
    let repo = DieselRepository::new(pool);
    let state = AppState::new(repo);
    Ok(router(state, HotReload::Off).into())
}
