use async_graphql::Object;

#[derive(Clone, Debug)]
pub struct ScrapedRecipe {
    scraped: recipe_scrape::ScrapedRecipe,
}

#[Object]
impl ScrapedRecipe {
    async fn name(&self) -> &str {
        &self.scraped.name
    }

    async fn source(&self) -> Option<&str> {
        Some(self.scraped.source.as_str())
    }

    async fn notes(&self) -> Option<&str> {
        self.scraped.notes.as_deref()
    }

    async fn prep_time_minutes(&self) -> Option<i32> {
        self.scraped.prep_time_minutes.map(|x| x as i32)
    }

    async fn cooking_time_minutes(&self) -> Option<i32> {
        self.scraped.cooking_time_minutes.map(|x| x as i32)
    }

    // async fn ingredients(&self, ctx: &Context<'_>) -> Result<Vec<RecipeIngredient>> {
    //     ctx.data_unchecked::<DataLoader<RecipeIngredientLoader>>()
    //         .load_one(self.model.id)
    //         .await
    //         .map(Option::unwrap_or_default)
    // }
}

impl From<recipe_scrape::ScrapedRecipe> for ScrapedRecipe {
    fn from(scraped: recipe_scrape::ScrapedRecipe) -> Self {
        Self { scraped }
    }
}
