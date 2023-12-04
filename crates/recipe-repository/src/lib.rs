use async_trait::async_trait;

use recipe_db::prelude::*;

mod recipe;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pool error")]
    Pool(#[from] PoolError),
    #[error("database error")]
    Database(#[from] DieselError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait Repository<T> {
    type Id;
    async fn get(&self, id: Self::Id) -> Result<Option<T>>;
    async fn list(&self) -> Result<Vec<T>>;
}

#[derive(Clone)]
pub struct DieselRepository {
    pool: DbPool,
}

impl std::fmt::Debug for DieselRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DieselRepository")
            .field("pool", &"<pool>")
            .finish()
    }
}

impl DieselRepository {
    pub fn new(pool: recipe_db::DbPool) -> Self {
        Self { pool }
    }
}
