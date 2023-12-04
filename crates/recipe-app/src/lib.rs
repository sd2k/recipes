#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use recipe_scrape::RecipeScraper;

use crate::server_functions::recipes;

pub mod server_functions;

#[derive(Props, PartialEq)]
struct SidebarProps {
    meal_plans: Vec<()>,
}

fn Sidebar(cx: Scope<SidebarProps>) -> Element {
    let recipes = use_server_future(cx, (), |()| async move { recipes().await.unwrap() })?;
    log::info!("recipes: {:?}", recipes.value());

    cx.render(rsx!(
        h1 { "Recipes" }
        ul {
            recipes.value().iter().map(|recipe| {
                rsx!(li { "{recipe.name}" })
            })
        }
        h1 { "Meal plans" }
    ))
}

#[derive(Props, PartialEq)]
struct ScrapedIngredientProps {
    ingredient: recipe_scrape::ScrapedIngredient,
}

fn ScrapedIngredient(cx: Scope<ScrapedIngredientProps>) -> Element {
    let ingredient = &cx.props.ingredient;
    let name = ingredient.name.as_deref().unwrap_or("unknown ingredient!");
    cx.render(rsx!(
        li {
            // span { "{ingredient.amount}" }
            // ingredient.unit.map(|u| rsx!(span { "{u}" }))
            span { "{name}" }
        }
    ))
}

#[derive(Props, PartialEq)]
struct ScrapedRecipeProps {
    recipe: recipe_scrape::ScrapedRecipe,
}

fn ScrapedRecipe(cx: Scope<ScrapedRecipeProps>) -> Element {
    cx.render(rsx!(
        h1 { "{cx.props.recipe.name}" }
        h2 { "Ingredients" }
        ul {
            cx.props.recipe.ingredients.iter().map(|ingredient| {
                rsx!(ScrapedIngredient { ingredient: ingredient.clone() })
            })
        }
    ))
}

fn Scraper(cx: Scope) -> Element {
    let scraper = use_state(cx, RecipeScraper::new);
    let url = use_state(cx, || "".to_string());
    let recipe = use_state(cx, || None);
    let scrape_recipe = move |_| {
        let scraper = scraper.to_owned();
        let recipe = recipe.to_owned();
        cx.spawn({
            let url = url.get().parse();
            async move {
                match url {
                    Ok(u) => match scraper.scrape(u).await {
                        Ok(r) => recipe.set(Some(r)),
                        Err(err) => log::error!("Failed to scrape recipe: {:?}", err),
                    },
                    Err(err) => log::error!("Invalid URL: {:?}", err),
                };
            }
        });
    };
    cx.render(rsx!(
        h1 { "Scrape a recipe" }
        input {
            name: "url",
            placeholder: "Recipe URL",
            value: "{url}",
            oninput: move |evt| url.set(evt.value.clone()),
        },
        input {
            r#type: "submit",
            onclick: scrape_recipe,
            "Scrape"
        }
        recipe.as_ref().map(|x| rsx!(ScrapedRecipe { recipe: x.clone() }))
    ))
}

pub fn app(cx: Scope) -> Element {
    log::info!("Rendering app");
    cx.render(rsx!(
        h1 { "Recipe planner" }
        div {
            // div {
            //     button { onclick: |_| recipes_fetch.send(()), "Refresh" }
            // }
            Sidebar { meal_plans: vec![] },
        }
        Scraper {}
    ))
}
