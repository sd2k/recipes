use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::{
        deadpool::{Pool, PoolBuilder},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use dotenvy::dotenv;

pub mod dsl;
pub mod models;
pub mod schema;
pub mod traits;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Pool<AsyncPgConnection>;

pub fn build_pool(database_url: &str) -> PoolBuilder<AsyncPgConnection> {
    let config = AsyncDieselConnectionManager::new(database_url);
    Pool::builder(config)
}

pub fn create_pool() -> anyhow::Result<DbPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(build_pool(&database_url).build()?)
}

pub mod prelude {
    pub use diesel_async::RunQueryDsl;

    pub use super::traits::*;
}
