use async_trait::async_trait;
use shuttle_service::{
    database::{AwsRdsEngine, Type},
    error::CustomError,
    Factory, ResourceBuilder,
};

use recipe_db::{build_pool, DbPool};

pub struct ShuttleDbPool;

#[async_trait]
impl ResourceBuilder<DbPool> for ShuttleDbPool {
    fn new() -> Self {
        Self
    }

    async fn build(
        self,
        factory: &mut dyn Factory,
        _runtime: &shuttle_service::Runtime,
    ) -> Result<DbPool, shuttle_service::Error> {
        let connection_string = factory
            .get_db_connection_string(Type::AwsRds(AwsRdsEngine::Postgres))
            .await?;
        Ok(build_pool(&connection_string)
            .max_size(5)
            .build()
            .map_err(CustomError::new)?)
    }
}
