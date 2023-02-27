#![allow(non_snake_case)]
use std::{cell::RefCell, time::Duration};

use dioxus::prelude::*;

use futures_util::{select, StreamExt};
use recipe_client::prelude::*;

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

pub fn app(cx: Scope<RootProps>) -> Element {
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
    ))
}
