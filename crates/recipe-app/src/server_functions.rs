use dioxus_fullstack::prelude::*;

use recipe_scrape::ScrapedRecipe;
use recipe_shared::Recipe;

#[server(Recipes)]
pub async fn recipes() -> Result<Vec<Recipe>, ServerFnError> {
    // let recipe_repo: Vec<Recipe> = extract();
    Ok(vec![Recipe {
        id: 1,
        created_at: chrono::Utc::now().naive_utc(),
        name: "something".to_string(),
        slug: "something".to_string(),
        source: None,
        notes: None,
        prep_time_minutes: None,
        cooking_time_minutes: Some(10),
    }])
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
