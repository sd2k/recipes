mod ingredient;
mod scrapers;

use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use reqwest::Url;

pub use ingredient::ScrapedIngredient;
use scrapers::SCRAPERS;

#[cfg(not(target_arch = "wasm32"))]
const RECIPE_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")"
);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("HTML parse error: {0}")]
    Html(io::Error),
    #[error("not a recipe")]
    NotARecipe,
    #[error("unsupported host {0}")]
    UnrecognisedHost(String),
    #[error("JSON error: {0}")]
    Scrape(#[from] scrapers::ScrapeError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScrapedRecipe {
    pub name: String,
    pub source: Url,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub prep_time_minutes: Option<u32>,
    pub cooking_time_minutes: Option<u32>,
    pub servings: Option<u32>,
    pub ingredients: Vec<ScrapedIngredient>,
}

#[derive(Clone, Debug)]
pub struct RecipeScraper {
    client: reqwest::Client,
}

impl RecipeScraper {
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut client = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        {
            client = client
                .user_agent(RECIPE_USER_AGENT)
                .timeout(Duration::from_secs(10));
        }
        Self::with_client(
            client
                .build()
                .expect("failed to build reqwest client; check host TLS config"),
        )
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn scrape(&self, url: Url) -> Result<ScrapedRecipe, Error> {
        let host = url.host_str().expect("fetched URL to have valid host");
        let scraper = SCRAPERS
            .get(host)
            .ok_or(Error::UnrecognisedHost(host.to_string()))?;
        let response = self
            .client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?;
        let text = response.text().await?;
        let html = webpage::HTML::from_string(text, Some(url.to_string())).map_err(Error::Html)?;

        html.schema_org
            .into_iter()
            .find(|schema| schema.schema_type.as_str() == "Recipe")
            .ok_or(Error::NotARecipe)
            .and_then(|schema| Ok(scraper.scrape(url, schema.value)?))
    }
}

impl Default for RecipeScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use crate::{RecipeScraper, RECIPE_USER_AGENT};

    #[test]
    fn user_agent() {
        // TODO: this will fail once we bump the version of the lib.
        assert_eq!(
            RECIPE_USER_AGENT,
            "recipe-scrape/0.1.0 (https://github.com/sd2k/recipes)"
        );
    }

    #[tokio::test]
    async fn scrape_bbc_good_food() {
        let url: Url = "https://www.bbcgoodfood.com/recipes/sausage-pasta-bake"
            .parse()
            .unwrap();
        let recipe = RecipeScraper::new()
            .scrape(url.clone())
            .await
            .expect("scrape should work");
        assert_eq!(recipe.name, "Sausage pasta bake");
        assert_eq!(recipe.source, url);
        assert_eq!(recipe.notes, None);
        assert_eq!(recipe.prep_time_minutes, Some(30));
        assert_eq!(recipe.cooking_time_minutes, Some(90));
        assert_eq!(recipe.servings, Some(4));
    }
}
