use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_query::prelude::*;

use recipe_shared::Recipe;

use crate::{
    components::ScrapedRecipe,
    hooks::{use_recipes, QueryValue},
    server::scrape_recipe,
};

pub fn RecipeGrid(cx: Scope) -> Element {
    let recipes = use_recipes(cx);
    cx.render(match recipes.result().value() {
        QueryResult::Ok(QueryValue::Recipes(rs)) => rsx!(
            div { class: "grid grid-cols-2 md:grid-cols-6 gap-4 p-4",
                rs.iter().map(|recipe| {
                    rsx!(RecipeCard { key: "{recipe.name}", recipe: recipe.clone() })
                })
            }
        ),
        QueryResult::Err(_) => rsx!(div { "error" }),
        QueryResult::Loading(_) => rsx!(div { "loading" }),
    })
}

#[component]
fn RecipeCard(cx: Scope, recipe: Recipe) -> Element {
    cx.render(rsx!(
        div { class: "card shadow-xl bg-primary text-primary-content",
            div { class: "card-body items-center text-center",
                recipe.image_url.as_ref().map(|url| rsx!(figure {
                    img { src: "{url}", alt: "Recipe image" }
                })),
                h4 { class: "card-title", "{cx.props.recipe.name}" }
                recipe.notes.as_ref().map(|n| rsx!(p { "{n}" }))
            }
        }
    ))
}

fn NewRecipeButton(cx: Scope) -> Element {
    cx.render(rsx!(
        button { class: "btn btn-circle fixed z-90 bottom-10 right-8", "onclick": "new_recipe.showModal()",
            svg {
                class: "h-5 w-5",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M12 6v6m0 0v6m0-6h6m-6 0H6"
                }
            }
        }
    ))
}

fn NewRecipeModal(cx: Scope) -> Element {
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
    cx.render(html!(
        <dialog id="new_recipe" class="modal">
            <div class="modal-box w-9/12 max-w-5xl">
                <h3 class="font-bold text-lg">"Add a recipe"</h3>
                <div>
                    <label "for"="recipe_url" class="py-4">"Recipe URL"</label>
                    <input
                        id="recipe_url"
                        "type"="text"
                        name="url"
                        placeholder="https://www.bbcgoodfood.com/recipes/sausage-pasta-bake"
                        oninput={move |evt| url.set(evt.value.clone())}
                    />
                </div>
                <button "type"="submit" class="btn btn-primary" onclick={scrape_recipe}>"Scrape"</button>
                {recipe.as_ref().map(|x| rsx!(ScrapedRecipe { recipe: x.clone() }))}
            </div>
            <form method="dialog" class="modal-backdrop">
                <button>"close"</button>
            </form>
        </dialog>
    ))
}

pub fn RecipesPage(cx: Scope) -> Element {
    cx.render(html!(
        <div>
            <RecipeGrid />
            <NewRecipeModal />
            <NewRecipeButton />
        </div>
    ))
}
