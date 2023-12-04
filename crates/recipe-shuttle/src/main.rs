#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(recipe_server::router().into())
}
