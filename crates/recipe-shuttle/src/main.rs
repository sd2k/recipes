use recipe_app::server::AppState;
use recipe_db::create_pool;
use recipe_repository::DieselRepository;
use recipe_server::router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let pool = create_pool().expect("create db pool");
    let repo = DieselRepository::new(pool);
    let state = AppState::new(repo);
    Ok(router(state).into())
}
