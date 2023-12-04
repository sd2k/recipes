mod ingredient;
#[cfg(feature = "scraper")]
mod scraper;

use serde::{Deserialize, Serialize};
use url::Url;

pub use ingredient::ScrapedIngredient;
#[cfg(feature = "scraper")]
pub use scraper::RecipeScraper;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
