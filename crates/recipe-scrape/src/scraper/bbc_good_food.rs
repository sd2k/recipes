use iso8601_duration::Duration;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    scraper::{Error, Scraper},
    ScrapedRecipe,
};

pub struct BBCGoodFoodScraper;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Yield {
    String(String),
    Number(u32),
}

static YIELD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine|ten|\d+)"#).unwrap());

impl Yield {
    fn as_u32(&self) -> Option<u32> {
        match self {
            Self::String(s) => YIELD_REGEX.find(s).and_then(|x| {
                let substr = x.as_str();
                match substr.parse::<u32>() {
                    Ok(n) => Some(n),
                    Err(_) => match substr {
                        "one" => Some(1),
                        "two" => Some(2),
                        "three" => Some(3),
                        "four" => Some(4),
                        "five" => Some(5),
                        "six" => Some(6),
                        "seven" => Some(7),
                        "eight" => Some(8),
                        "nine" => Some(9),
                        "ten" => Some(10),
                        _ => None,
                    },
                }
            }),
            Self::Number(n) => Some(*n),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BBCGoodFoodRecipe {
    name: String,
    description: String,
    cook_time: Duration,
    prep_time: Duration,
    recipe_yield: Yield,
    #[serde(alias = "recipeIngredient")]
    ingredients: Vec<String>,
    image: Option<Image>,
}

impl Scraper for BBCGoodFoodScraper {
    fn host(&self) -> &'static str {
        "www.bbcgoodfood.com"
    }

    fn scrape(&self, url: Url, value: serde_json::Value) -> Result<ScrapedRecipe, Error> {
        let recipe: BBCGoodFoodRecipe = serde_json::from_value(value)?;
        Ok(ScrapedRecipe {
            name: recipe.name,
            description: Some(recipe.description),
            source: url,
            notes: None,
            prep_time_minutes: recipe.prep_time.num_minutes().map(|x| x.ceil() as u32),
            cooking_time_minutes: recipe.cook_time.num_minutes().map(|x| x.ceil() as u32),
            servings: recipe.recipe_yield.as_u32(),
            ingredients: recipe
                .ingredients
                .into_iter()
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?,
            image_url: recipe.image.map(|x| x.url),
        })
    }
}
