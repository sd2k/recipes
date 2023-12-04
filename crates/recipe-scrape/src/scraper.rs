use std::{collections::HashMap, io, time::Duration};

use once_cell::sync::Lazy;
use reqwest::Url;
use serde::de::Error as DeError;

mod bbc_good_food;

use crate::{ingredient, ScrapedRecipe};

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
    Json(#[from] serde_json::Error),
    #[error("parsing ingredients: {0}")]
    Ingredient(#[from] ingredient::Error),
}

#[derive(Clone, Debug)]
pub struct RecipeScraper {
    client: reqwest::Client,
}

impl RecipeScraper {
    const USER_AGENT: &'static str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("CARGO_PKG_REPOSITORY"),
        ")"
    );

    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(Self::USER_AGENT)
            .timeout(Duration::from_secs(10));
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
            .and_then(|schema| scraper.scrape(url, schema.value))
    }
}

impl Default for RecipeScraper {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Scraper: Sync + Send {
    fn host(&self) -> &'static str;
    fn scrape(&self, url: Url, value: serde_json::Value) -> Result<ScrapedRecipe, Error>;
}

pub struct DummyScraper;

impl Scraper for DummyScraper {
    fn host(&self) -> &'static str {
        "dummy"
    }

    fn scrape(&self, _url: Url, _value: serde_json::Value) -> Result<ScrapedRecipe, Error> {
        Err(Error::Json(serde_json::Error::custom("dummy scraper")))
    }
}

pub static SCRAPERS: Lazy<HashMap<&'static str, &'static dyn Scraper>> = Lazy::new(|| {
    [
        &bbc_good_food::BBCGoodFoodScraper as &dyn Scraper,
        &DummyScraper,
    ]
    .map(|s| (s.host(), s))
    .into()
});

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use super::RecipeScraper;

    #[test]
    fn user_agent() {
        // TODO: this will fail once we bump the version of the lib.
        assert_eq!(
            RecipeScraper::USER_AGENT,
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
