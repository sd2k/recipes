use dioxus::prelude::*;

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
pub struct ScrapedRecipeProps {
    recipe: recipe_scrape::ScrapedRecipe,
}

pub fn ScrapedRecipe(cx: Scope<ScrapedRecipeProps>) -> Element {
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
