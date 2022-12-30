use std::collections::HashMap;

use async_graphql::{
    async_trait::async_trait,
    dataloader::{DataLoader, Loader},
    Context, Error, Object, Result, ID,
};
use futures_util::TryStreamExt;

use recipe_db::{
    models::{self, IngredientId},
    prelude::*,
    DbPool,
};

use crate::{
    types::{Measurement, MeasurementLoader, RecipeIngredient, RecipeIngredientLoader},
    HasModel,
};

#[derive(Clone, Debug)]
pub struct Ingredient {
    model: models::Ingredient,
}

#[Object]
impl Ingredient {
    async fn id(&self) -> ID {
        self.model.id.into()
    }

    async fn name(&self) -> &str {
        &self.model.name
    }

    async fn slug(&self) -> &str {
        &self.model.slug
    }

    pub async fn default_measurement(&self, ctx: &Context<'_>) -> Result<Option<Measurement>> {
        crate::load_one::<MeasurementLoader, _>(ctx, self.model.default_measurement_id).await
    }

    pub async fn recipe_ingredients(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Vec<RecipeIngredient>>> {
        ctx.data_unchecked::<DataLoader<RecipeIngredientLoader>>()
            .load_one(self.model.id)
            .await
    }
}

impl HasModel<'_> for Ingredient {
    type Model = models::Ingredient;
    fn from_model(model: Self::Model) -> Self {
        Self { model }
    }
}

pub struct IngredientLoader(DbPool);

impl IngredientLoader {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<IngredientId> for IngredientLoader {
    type Value = Ingredient;
    type Error = Error;

    async fn load(&self, keys: &[IngredientId]) -> Result<HashMap<IngredientId, Self::Value>> {
        Ok(models::Ingredient::by_ids(keys)
            .load_stream::<models::Ingredient>(&mut self.0.get().await?)
            .await?
            .map_ok(|model| (model.id, Ingredient { model }))
            .try_collect()
            .await?)
    }
}
