#![allow(non_snake_case)]
use std::{cell::RefCell, time::Duration};

use dioxus::prelude::*;

use futures_util::{select, StreamExt};
use recipe_client::prelude::*;
use recipe_scrape::RecipeScraper;

#[derive(Props, PartialEq)]
pub struct RootProps {
    pub origin: &'static str,
    pub initial_state: RefCell<Option<AllRecipes>>,
}

impl Default for RootProps {
    fn default() -> Self {
        Self {
            origin: "http://localhost:8000",
            initial_state: None.into(),
        }
    }
}

#[derive(Props, PartialEq)]
struct SidebarProps<'a> {
    recipes: &'a [Recipe],
    meal_plans: &'a [()],
}

fn Sidebar<'a>(cx: Scope<'a, SidebarProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        h1 { "Recipes" }
        ul {
            cx.props.recipes.iter().map(|recipe| {
                rsx!(li { "{recipe.name}" })
            })
        }
        h1 { "Meal plans" }
    ))
}

#[derive(Props, PartialEq)]
struct ScrapedIngredientProps<'a> {
    ingredient: &'a recipe_scrape::ScrapedIngredient,
}

fn ScrapedIngredient<'a>(cx: Scope<'a, ScrapedIngredientProps<'a>>) -> Element<'a> {
    let ingredient = cx.props.ingredient;
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
struct ScrapedRecipeProps<'a> {
    recipe: &'a recipe_scrape::ScrapedRecipe,
}

fn ScrapedRecipe<'a>(cx: Scope<'a, ScrapedRecipeProps<'a>>) -> Element<'a> {
    // let client = use_context(cx);
    // let add_recipe = move |_| {
    //     let recipe = cx.props.recipe;
    //     cx.
    // };
    cx.render(rsx!(
        h1 { "{cx.props.recipe.name}" }
        h2 { "Ingredients" }
        ul {
            cx.props.recipe.ingredients.iter().map(|ingredient| {
                rsx!(ScrapedIngredient { ingredient: ingredient })
            })
        }
        // input {
        //     r#type: "submit",
        //     onclick: add_recipe,
        // }
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
        recipe.as_ref().map(|x| rsx!(ScrapedRecipe { recipe: x }))
    ))
}

pub struct Client {
    client: reqwest::Client,
    url: reqwest::Url,
}

impl Client {
    pub fn new(origin: &'static str) -> Self {
        let mut url = reqwest::Url::parse(origin).expect("origin should be a valid URL");
        url.set_path("/graphql");
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }
}

pub fn app(cx: Scope<RootProps>) -> Element {
    use_shared_state_provider(cx, || Client::new(cx.props.origin));
    log::info!("Rendering app");
    let recipes = use_state(cx, || cx.props.initial_state.replace(None));
    let recipes_fetch = use_coroutine(cx, |mut rx: UnboundedReceiver<()>| {
        let mut url = reqwest::Url::parse(cx.props.origin).unwrap();
        url.set_path("/graphql");
        let mut interval = fluvio_wasm_timer::Interval::new(Duration::from_secs(5)).fuse();
        let recipes = recipes.clone();
        async move {
            if let Ok(new) = reqwest::Client::new()
                .post(url.clone())
                .run_graphql(AllRecipes::build(()))
                .await
                .map(|x| x.data)
            {
                recipes.modify(|_| new);
                loop {
                    select! {
                        _ = rx.next() => {
                            if let Ok(Some(new)) = reqwest::Client::new()
                                .post(url.clone())
                                .run_graphql(AllRecipes::build(()))
                                .await.map(|x| x.data)
                            {
                                recipes.modify(|_| Some(new));
                            }
                        },
                        _ = interval.next() => {
                            if let Ok(Some(new)) = reqwest::Client::new()
                                .post(url.clone())
                                .run_graphql(AllRecipes::build(()))
                                .await.map(|x| x.data)
                            {
                                recipes.modify(|_| Some(new));
                            }
                        }
                    }
                }
            }
        }
    });

    let recipes = recipes
        .as_ref()
        .map(|r| r.recipes.as_slice())
        .unwrap_or_default();
    cx.render(rsx!(
        h1 { "Recipe planner" }
        div {
            div {
                button { onclick: |_| recipes_fetch.send(()), "Refresh" }
            }
            Sidebar { meal_plans: &[], recipes: recipes },
        }
        Scraper {}
    ))
}
