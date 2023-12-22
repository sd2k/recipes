use dioxus::prelude::*;

#[component]
fn ScrapedIngredient(cx: Scope, ingredient: recipe_scrape::ScrapedIngredient) -> Element {
    cx.render(rsx!(
        li {
            span { "{ingredient}" }
        }
    ))
}

#[component]
pub fn ScrapedRecipe(cx: Scope, recipe: recipe_scrape::ScrapedRecipe) -> Element {
    cx.render(rsx!(
        h1 { "{recipe.name}" }
        h2 { "Ingredients" }
        ul {
            recipe.ingredients.iter().map(|ingredient| {
                rsx!(ScrapedIngredient { ingredient: ingredient.clone() })
            })
        }
    ))
}
