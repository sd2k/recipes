use anyhow::{Context, Result};
use diesel::prelude::*;

use recipe_db::*;

use crate::prelude::*;

pub fn create_measurement(
    connection: &mut PgConnection,
    measurement: models::NewMeasurement<'_>,
) -> Result<models::Measurement> {
    use schema::measurements;
    diesel::insert_into(measurements::table)
        .values(&measurement)
        .on_conflict(measurements::slug)
        .do_update()
        .set(&measurement)
        .get_result(connection)
        .context("creating measurement")
}

pub fn create_ingredient(
    connection: &mut PgConnection,
    ingredient: models::NewIngredient<'_>,
) -> Result<models::Ingredient> {
    use schema::ingredients;
    diesel::insert_into(ingredients::table)
        .values(&ingredient)
        .on_conflict(ingredients::slug)
        .do_update()
        .set(&ingredient)
        .get_result(connection)
        .context("creating ingredient")
}

pub fn create_recipe(
    connection: &mut PgConnection,
    recipe: models::NewRecipe<'_>,
    ingredients: Vec<models::NewRecipeIngredient<'_>>,
) -> Result<(models::Recipe, Vec<models::RecipeIngredient>)> {
    connection.transaction(|conn| {
        let recipe: models::Recipe = diesel::insert_into(schema::recipes::table)
            .values(&recipe)
            .get_result(conn)
            .context("creating recipe")?;

        let recipe_ingredients = ingredients
            .into_iter()
            .map(|mut ingredient| {
                ingredient.recipe_id = recipe.id.into();
                diesel::insert_into(schema::recipe_ingredients::table)
                    .values(ingredient)
                    .get_result::<models::RecipeIngredient>(conn)
                    .context("creating recipe ingredient")
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((recipe, recipe_ingredients))
    })
}

fn main() -> Result<()> {
    use self::schema::recipes::dsl::*;

    let connection = &mut establish_connection();

    let grams = create_measurement(
        connection,
        models::NewMeasurement {
            name: "grams",
            abbreviation: Some("g"),
        },
    )?;
    let tablespoon = create_measurement(
        connection,
        models::NewMeasurement {
            name: "tablespoons",
            abbreviation: Some("tbsp"),
        },
    )?;
    let count = create_measurement(
        connection,
        models::NewMeasurement {
            name: "count",
            abbreviation: None,
        },
    )?;

    let onion = create_ingredient(
        connection,
        models::NewIngredient {
            name: "Onion",
            default_measurement_id: count.id,
        },
    )?;
    let tomato_puree = create_ingredient(
        connection,
        models::NewIngredient {
            name: "Tomato Puree",
            default_measurement_id: tablespoon.id,
        },
    )?;
    let garlic_clove = create_ingredient(
        connection,
        models::NewIngredient {
            name: "Garlic clove",
            default_measurement_id: count.id,
        },
    )?;
    let spaghetti = create_ingredient(
        connection,
        models::NewIngredient {
            name: "Spaghetti",
            default_measurement_id: grams.id,
        },
    )?;

    create_recipe(
        connection,
        models::NewRecipe {
            name: "Spaghetti with Tomato Sauce",
            source: Some("Anna"),
            notes: Some("Delicious!"),
            prep_time_minutes: Some(15),
            cooking_time_minutes: Some(15),
        },
        vec![
            models::NewRecipeIngredient {
                ingredient_id: onion.id,
                recipe_id: 0.into(),
                measurement_id: None,
                quantity: 1.into(),
                index: 0,
                notes: None,
            },
            models::NewRecipeIngredient {
                ingredient_id: garlic_clove.id,
                recipe_id: 0.into(),
                measurement_id: None,
                quantity: 2.into(),
                index: 0,
                notes: None,
            },
            models::NewRecipeIngredient {
                ingredient_id: tomato_puree.id,
                recipe_id: 0.into(),
                measurement_id: None,
                quantity: 1.into(),
                index: 0,
                notes: None,
            },
            models::NewRecipeIngredient {
                ingredient_id: spaghetti.id,
                recipe_id: 0.into(),
                measurement_id: None,
                quantity: 500.into(),
                index: 1,
                notes: None,
            },
        ],
    )
    .ok();

    let results = recipes.limit(5).load::<models::Recipe>(connection)?;

    println!(
        "Displaying {} recipe{}\n",
        results.len(),
        (results.len() != 1).then_some("s").unwrap_or("")
    );
    for recipe in results {
        println!("# {}", recipe.name);
        println!("-----------\n");
        println!("## Ingredients\n");
        recipe
            .ingredients()
            .load::<models::RecipeIngredient>(connection)?
            .into_iter()
            .map(|recipe_ingredient| {
                let ingredient = recipe_ingredient.ingredient(connection)?;
                let measurement = recipe_ingredient.measurement(connection, &ingredient)?;
                println!(
                    "- {}{} {}{}",
                    recipe_ingredient.quantity,
                    measurement.abbreviation.as_deref().unwrap_or(""),
                    ingredient.name,
                    (measurement.name == "count" && recipe_ingredient.quantity > 1.into())
                        .then_some("s")
                        .unwrap_or("")
                );
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        if let Some(n) = recipe.notes {
            println!("\n## Notes\n\n{}", n);
        }
    }

    let results = models::Recipe::by_ids(&[1.into(), 2.into()]);
    Ok(())
}
