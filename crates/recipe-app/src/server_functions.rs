use dioxus_fullstack::prelude::*;

use recipe_shared::Recipe;

#[server(Recipes)]
pub async fn recipes() -> Result<Vec<Recipe>, ServerFnError> {
    // let recipe_repo: Vec<Recipe> = extract();
    Ok(vec![Recipe {
        id: 1,
        created_at: chrono::Utc::now().naive_utc(),
        name: "something".to_string(),
        slug: "something".to_string(),
        source: None,
        notes: None,
        prep_time_minutes: None,
        cooking_time_minutes: Some(10),
    }])
}
