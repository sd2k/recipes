use async_trait::async_trait;
use serde::Serialize;
use shuttle_common::{database, resource, DbInput, DbOutput};
use shuttle_runtime::{Error, Factory, ResourceBuilder};

use recipe_db::{build_pool, DbPool};

#[derive(Serialize)]
pub struct ShuttleDbPool {
    config: DbInput,
}

#[async_trait]
impl ResourceBuilder<DbPool> for ShuttleDbPool {
    fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    const TYPE: resource::Type =
        resource::Type::Database(database::Type::Shared(database::SharedEngine::Postgres));

    type Config = DbInput;

    type Output = DbOutput;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        let info = match factory.get_metadata().env {
            shuttle_service::Environment::Deployment => DbOutput::Info(
                factory
                    .get_db_connection(database::Type::Shared(database::SharedEngine::Postgres))
                    .await?,
            ),
            shuttle_service::Environment::Local => {
                if let Some(local_uri) = self.config.local_uri {
                    DbOutput::Local(local_uri)
                } else {
                    DbOutput::Info(
                        factory
                            .get_db_connection(database::Type::Shared(
                                database::SharedEngine::Postgres,
                            ))
                            .await?,
                    )
                }
            }
        };

        Ok(info)
    }

    async fn build(build_data: &Self::Output) -> Result<DbPool, Error> {
        let connection_string = match build_data {
            DbOutput::Local(local_uri) => local_uri.clone(),
            DbOutput::Info(info) => info.connection_string_private(),
        };
        Ok(build_pool(&connection_string)
            .max_size(5)
            .build()
            .map_err(|e| Error::Database(e.to_string()))?)
    }
}
