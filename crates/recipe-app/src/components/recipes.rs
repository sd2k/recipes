use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_html_macro::html;

use recipe_shared::Recipe;

use crate::server::recipes;

#[derive(Props, PartialEq)]
pub struct RecipeGridProps {}

pub fn RecipeGrid(cx: Scope<RecipeGridProps>) -> Element {
    let recipes = use_server_future(cx, (), |()| async move { recipes().await.unwrap() })?;
    log::info!("recipes: {:?}", recipes.value());

    cx.render(rsx!(
        div { class: "grid grid-cols-2 md:grid-cols-6 gap-4 p-4",
            recipes.value().iter().map(|recipe| {
                rsx!(RecipeCard { key: "{recipe.name}", recipe: recipe.clone() })
            })
        }
    ))
}

#[derive(Props, PartialEq)]
struct RecipeCardProps {
    recipe: Recipe,
}

fn RecipeCard(cx: Scope<RecipeCardProps>) -> Element {
    cx.render(rsx!(
        div { class: "card shadow-xl bg-primary text-primary-content",
            div { class: "card-body items-center text-center",
                cx.props.recipe.image_url.as_ref().map(|url| rsx!(figure {
                    img { src: "{url}", alt: "Recipe image" }
                })),
                h4 { class: "card-title", "{cx.props.recipe.name}" }
                cx.props.recipe.notes.as_ref().map(|n| rsx!(p { "{n}" }))
            }
        }
    ))
}

pub fn RecipesPage(cx: Scope) -> Element {
    cx.render(html!(
        <div>
            <div class="container mx-auto">
                <RecipeGrid />
            </div>
            <dialog id="new-recipe" class="modal">
            </dialog>
        </div>
    ))
}
