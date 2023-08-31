use std::collections::HashMap;

use once_cell::sync::Lazy;
use reqwest::Url;
use serde::de::Error;

mod bbc_good_food;

use crate::{ingredient, ScrapedRecipe};

#[derive(Debug, thiserror::Error)]
pub enum ScrapeError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("parsing ingredients: {0}")]
    Ingredient(#[from] ingredient::Error),
}

pub trait Scraper: Sync + Send {
    fn host(&self) -> &'static str;
    fn scrape(&self, url: Url, value: serde_json::Value) -> Result<ScrapedRecipe, ScrapeError>;
}

pub struct DummyScraper;

impl Scraper for DummyScraper {
    fn host(&self) -> &'static str {
        "dummy"
    }

    fn scrape(&self, _url: Url, _value: serde_json::Value) -> Result<ScrapedRecipe, ScrapeError> {
        Err(ScrapeError::Json(serde_json::Error::custom(
            "dummy scraper",
        )))
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
