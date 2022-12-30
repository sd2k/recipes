use std::collections::HashMap;

use async_graphql::{async_trait::async_trait, dataloader::Loader, Error, Object, Result, ID};
use futures_util::TryStreamExt;

use recipe_db::{
    models::{self, MeasurementId},
    prelude::*,
    DbPool,
};

use crate::HasModel;

#[derive(Clone, Debug)]
pub struct Measurement {
    model: models::Measurement,
}

#[Object]
impl Measurement {
    async fn id(&self) -> ID {
        self.model.id.into()
    }

    async fn name(&self) -> &str {
        &self.model.name
    }

    async fn slug(&self) -> &str {
        &self.model.slug
    }

    async fn abbreviation(&self) -> Option<&str> {
        self.model.abbreviation.as_deref()
    }
}

impl HasModel<'_> for Measurement {
    type Model = models::Measurement;
    fn from_model(model: Self::Model) -> Self {
        Self { model }
    }
}

pub struct MeasurementLoader(DbPool);

impl MeasurementLoader {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<MeasurementId> for MeasurementLoader {
    type Value = Measurement;
    type Error = Error;

    async fn load(&self, keys: &[MeasurementId]) -> Result<HashMap<MeasurementId, Self::Value>> {
        Ok(models::Measurement::by_ids(keys)
            .load_stream::<models::Measurement>(&mut self.0.get().await?)
            .await?
            .map_ok(|model| (model.id, Measurement { model }))
            .try_collect()
            .await?)
    }
}
