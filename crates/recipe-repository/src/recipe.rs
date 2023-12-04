use recipe_db::prelude::*;

use async_trait::async_trait;
use diesel::prelude::*;
use futures_util::TryStreamExt;

use recipe_db::models::{Recipe as DbRecipe, RecipeId};
use recipe_shared::Recipe as SharedRecipe;

use crate::{DieselRepository, Repository, Result};

struct Recipe(SharedRecipe);

impl From<DbRecipe> for Recipe {
    fn from(db_recipe: DbRecipe) -> Self {
        Self(SharedRecipe {
            id: db_recipe.id.into(),
            created_at: db_recipe.created_at,
            name: db_recipe.name,
            slug: db_recipe.slug,
            source: db_recipe.source,
            notes: db_recipe.notes,
            prep_time_minutes: db_recipe.prep_time_minutes,
            cooking_time_minutes: db_recipe.cooking_time_minutes,
        })
    }
}

#[async_trait]
impl Repository<SharedRecipe> for DieselRepository {
    type Id = RecipeId;
    async fn get(&self, id: Self::Id) -> Result<Option<SharedRecipe>> {
        let mut conn = self.pool.get().await?;
        let recipe: Option<DbRecipe> = DbRecipe::by_id(id).first(&mut conn).await.optional()?;
        Ok(recipe.map(|r| Recipe::from(r).0))
    }

    async fn list(&self) -> Result<Vec<SharedRecipe>> {
        let mut conn = self.pool.get().await?;
        let recipes = DbRecipe::all().load_stream(&mut conn).await?;
        Ok(recipes.map_ok(|r| Recipe::from(r).0).try_collect().await?)
    }
}
