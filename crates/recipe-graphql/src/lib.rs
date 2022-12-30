use async_graphql::{
    dataloader::{DataLoader, Loader},
    Context, EmptyMutation, EmptySubscription, Error, Object, Result, ID,
};

use futures_util::TryStreamExt;

use recipe_db::{
    models::{self, IngredientId, RecipeId},
    prelude::RunQueryDsl,
    traits::All,
    DbPool,
};

mod types;

use types::*;

pub struct QueryRoot;

impl QueryRoot {
    async fn load_one<T, I>(&self, ctx: &Context<'_>, id: ID) -> Result<Option<T::Value>>
    where
        T: Loader<I, Error = Error>,
        I: From<i64> + Send + Sync + Clone + std::hash::Hash + Eq + 'static,
    {
        ctx.data_unchecked::<DataLoader<T>>()
            .load_one(I::from(i64::try_from(id)?))
            .await
    }
}

#[Object]
impl QueryRoot {
    async fn recipe(&self, ctx: &Context<'_>, id: ID) -> Result<Option<types::Recipe>> {
        self.load_one::<RecipeLoader, RecipeId>(ctx, id).await
    }

    async fn ingredient(&self, ctx: &Context<'_>, id: ID) -> Result<Option<types::Ingredient>> {
        self.load_one::<IngredientLoader, IngredientId>(ctx, id)
            .await
    }

    async fn recipes(&self, ctx: &Context<'_>) -> Result<Vec<types::Recipe>> {
        Ok(models::Recipe::all()
            .load_stream::<models::Recipe>(&mut ctx.data_unchecked::<DbPool>().get().await?)
            .await?
            .map_ok(types::Recipe::from_model)
            .try_collect()
            .await?)
    }

    async fn ingredients(&self, ctx: &Context<'_>) -> Result<Vec<types::Ingredient>> {
        Ok(models::Ingredient::all()
            .load_stream::<models::Ingredient>(&mut ctx.data_unchecked::<DbPool>().get().await?)
            .await?
            .map_ok(types::Ingredient::from_model)
            .try_collect()
            .await?)
    }

    async fn measurements(&self, ctx: &Context<'_>) -> Result<Vec<types::Measurement>> {
        Ok(models::Measurement::all()
            .load_stream::<models::Measurement>(&mut ctx.data_unchecked::<DbPool>().get().await?)
            .await?
            .map_ok(types::Measurement::from_model)
            .try_collect()
            .await?)
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(pool: recipe_db::DbPool) -> Schema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .data(DataLoader::new(
            RecipeLoader::new(pool.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            RecipeIngredientLoader::new(pool.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            MeasurementLoader::new(pool.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(IngredientLoader::new(pool), tokio::spawn))
        .finish()
}

pub async fn load_one<T, I>(ctx: &Context<'_>, id: I) -> Result<Option<T::Value>>
where
    T: Loader<I, Error = Error>,
    I: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    ctx.data_unchecked::<DataLoader<T>>().load_one(id).await
}

pub trait HasModel<'a> {
    type Model;
    fn from_model(model: Self::Model) -> Self;
}
