#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    components::{Header, RecipeGrid, ScrapedRecipe},
    server::scrape_recipe,
};

mod components;
pub mod server;

fn Scraper(cx: Scope) -> Element {
    let url = use_state(cx, || "".to_string());
    let recipe = use_state::<Option<recipe_scrape::ScrapedRecipe>>(cx, || None);
    let scrape_recipe = move |_| {
        let url = url.to_owned();
        let recipe = recipe.to_owned();
        cx.spawn({
            async move {
                match scrape_recipe(url.to_string()).await {
                    Ok(r) => recipe.set(Some(r)),
                    Err(err) => log::error!("Failed to scrape recipe: {:?}", err),
                }
            }
        });
    };
    cx.render(rsx!(
        h1 { "Scrape a recipe" }
        input {
            name: "url",
            placeholder: "Recipe URL",
            value: "{url}",
            oninput: move |evt| url.set(evt.value.clone())
        }
        button { "type": "submit", onclick: scrape_recipe, "Scrape" }
        recipe.as_ref().map(|x| rsx!(ScrapedRecipe { recipe: x.clone() }))
    ))
}

pub fn app(cx: Scope) -> Element {
    log::info!("Rendering app");
    cx.render(rsx!(
        Header {}
        RecipeGrid {}
    ))
}
