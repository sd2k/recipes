use dioxus_fullstack::prelude::*;
use tracing::debug;

#[cfg(feature = "ssr")]
use recipe_repository::Repository;
use recipe_scrape::ScrapedRecipe;
use recipe_shared::Recipe;

#[cfg(feature = "ssr")]
use super::AppState;

#[server(Recipes)]
pub async fn recipes() -> Result<Vec<Recipe>, ServerFnError> {
    debug!("loading state from server context");
    let state: AppState = server_context()
        .get::<AppState>()
        .ok_or_else(|| ServerFnError::ServerError("missing state".to_string()))?;
    debug!("loading recipes from DB");
    Ok(state.repo.list().await?)
}

#[server(ScrapeRecipe)]
pub async fn scrape_recipe(url: String) -> Result<ScrapedRecipe, ServerFnError> {
    let scraper = recipe_scrape::RecipeScraper::new();
    let url = url.parse()?;
    let recipe = scraper.scrape(url).await?;
    Ok(recipe)
}

#[cfg(feature = "ssr")]
pub fn register_explicit() {
    let _ = Recipes::register_explicit();
    let _ = ScrapeRecipe::register_explicit();
}
