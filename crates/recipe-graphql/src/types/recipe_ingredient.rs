use std::collections::HashMap;

use async_graphql::{
    async_trait::async_trait, dataloader::Loader, Context, Error, Object, Result, ID,
};
use bigdecimal::ToPrimitive;
use futures_util::TryStreamExt;

use recipe_db::{
    models::{self, IngredientId, RecipeId},
    prelude::*,
    DbPool,
};

use crate::{
    types::{Ingredient, IngredientLoader, Measurement, MeasurementLoader, Recipe, RecipeLoader},
    HasModel,
};

#[derive(Debug, Clone)]
pub struct RecipeIngredient {
    model: models::RecipeIngredient,
}

#[Object]
impl RecipeIngredient {
    async fn recipe_id(&self) -> ID {
        self.model.recipe_id.into()
    }

    async fn ingredient_id(&self) -> ID {
        self.model.ingredient_id.into()
    }

    async fn recipe(&self, ctx: &Context<'_>) -> Result<Option<Recipe>> {
        crate::load_one::<RecipeLoader, _>(ctx, self.model.recipe_id).await
    }

    async fn ingredient(&self, ctx: &Context<'_>) -> Result<Option<Ingredient>> {
        crate::load_one::<IngredientLoader, _>(ctx, self.model.ingredient_id).await
    }

    async fn quantity(&self) -> Option<f32> {
        self.model.quantity.to_f32()
    }

    async fn measurement(&self, ctx: &Context<'_>) -> Result<Option<Measurement>> {
        if let Some(id) = self.model.measurement_id {
            crate::load_one::<MeasurementLoader, _>(ctx, id).await
        } else {
            match self.ingredient(ctx).await? {
                Some(ingredient) => ingredient.default_measurement(ctx).await,
                None => Ok(None),
            }
        }
    }
}

impl HasModel<'_> for RecipeIngredient {
    type Model = models::RecipeIngredient;
    fn from_model(model: Self::Model) -> Self {
        Self { model }
    }
}

pub struct RecipeIngredientLoader(DbPool);

impl RecipeIngredientLoader {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<RecipeId> for RecipeIngredientLoader {
    type Value = Vec<RecipeIngredient>;
    type Error = Error;

    async fn load(&self, keys: &[RecipeId]) -> Result<HashMap<RecipeId, Self::Value>> {
        Ok(models::RecipeIngredient::belonging_to_recipes(keys)
            .load_stream::<models::RecipeIngredient>(&mut self.0.get().await?)
            .await?
            .try_fold(HashMap::with_capacity(keys.len()), |mut acc, model| {
                acc.entry(model.recipe_id)
                    .or_insert_with(Vec::new)
                    .push(RecipeIngredient { model });
                futures_util::future::ready(Ok(acc))
            })
            .await?)
    }
}

#[async_trait]
impl Loader<IngredientId> for RecipeIngredientLoader {
    type Value = Vec<RecipeIngredient>;
    type Error = Error;

    async fn load(&self, keys: &[IngredientId]) -> Result<HashMap<IngredientId, Self::Value>> {
        Ok(models::RecipeIngredient::belonging_to_ingredients(keys)
            .load_stream::<models::RecipeIngredient>(&mut self.0.get().await?)
            .await?
            .try_fold(HashMap::with_capacity(keys.len()), |mut acc, model| {
                acc.entry(model.ingredient_id)
                    .or_insert_with(Vec::new)
                    .push(RecipeIngredient { model });
                futures_util::future::ready(Ok(acc))
            })
            .await?)
    }
}
