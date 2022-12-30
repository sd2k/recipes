use std::collections::HashMap;

use async_graphql::{
    async_trait::async_trait,
    dataloader::{DataLoader, Loader},
    Context, Error, Object, Result, ID,
};
use futures_util::TryStreamExt;

use recipe_db::{
    models::{self, RecipeId},
    prelude::*,
    DbPool,
};

use crate::{
    types::{RecipeIngredient, RecipeIngredientLoader},
    HasModel,
};

#[derive(Clone, Debug)]
pub struct Recipe {
    model: models::Recipe,
}

#[Object]
impl Recipe {
    async fn id(&self) -> ID {
        self.model.id.into()
    }

    async fn name(&self) -> &str {
        &self.model.name
    }

    async fn source(&self) -> Option<&str> {
        self.model.source.as_deref()
    }

    async fn notes(&self) -> Option<&str> {
        self.model.notes.as_deref()
    }

    async fn prep_time_minutes(&self) -> Option<i32> {
        self.model.prep_time_minutes
    }

    async fn cooking_time_minutes(&self) -> Option<i32> {
        self.model.cooking_time_minutes
    }

    async fn ingredients(&self, ctx: &Context<'_>) -> Result<Vec<RecipeIngredient>> {
        ctx.data_unchecked::<DataLoader<RecipeIngredientLoader>>()
            .load_one(self.model.id)
            .await
            .map(Option::unwrap_or_default)
    }
}

impl HasModel<'_> for Recipe {
    type Model = models::Recipe;
    fn from_model(model: Self::Model) -> Self {
        Self { model }
    }
}

#[derive(Clone)]
pub struct RecipeLoader(DbPool);

impl RecipeLoader {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<RecipeId> for RecipeLoader {
    type Value = Recipe;
    type Error = Error;

    async fn load(&self, keys: &[RecipeId]) -> Result<HashMap<RecipeId, Self::Value>, Self::Error> {
        Ok(models::Recipe::by_ids(keys)
            .load_stream(&mut self.0.get().await?)
            .await?
            .map_ok(|model: models::Recipe| (model.id, Recipe { model }))
            .try_collect()
            .await?)
    }
}
