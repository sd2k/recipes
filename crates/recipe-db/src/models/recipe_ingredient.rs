use chrono::prelude::*;
use diesel::{
    backend::Backend,
    helper_types::{AsSelect, EqAny, Filter, Select},
    prelude::*,
};

use bigdecimal::BigDecimal;

use crate::{
    models::{Ingredient, IngredientId, Measurement, MeasurementId, Recipe, RecipeId},
    schema::{ingredients, measurements, recipe_ingredients, recipes},
    traits::All,
};

type FilteredByRecipeId<'a> =
    Filter<recipe_ingredients::table, EqAny<recipe_ingredients::recipe_id, &'a [RecipeId]>>;
type FilteredByIngredientId<'a> =
    Filter<recipe_ingredients::table, EqAny<recipe_ingredients::ingredient_id, &'a [IngredientId]>>;

#[derive(Clone, Debug, Queryable, Identifiable, Associations, AsChangeset, Selectable)]
#[diesel(
    primary_key(recipe_id, ingredient_id),
    belongs_to(Recipe),
    belongs_to(Ingredient)
)]
pub struct RecipeIngredient {
    pub recipe_id: RecipeId,
    pub ingredient_id: IngredientId,
    pub created_at: NaiveDateTime,
    pub quantity: BigDecimal,
    pub measurement_id: Option<MeasurementId>,
    #[diesel(column_name = "idx")]
    pub index: i32,
    pub notes: Option<String>,
}

impl RecipeIngredient {
    pub fn belonging_to_recipes(ids: &[RecipeId]) -> FilteredByRecipeId {
        recipe_ingredients::table.filter(recipe_ingredients::recipe_id.eq_any(ids))
    }

    pub fn belonging_to_ingredients(ids: &[IngredientId]) -> FilteredByIngredientId {
        recipe_ingredients::table.filter(recipe_ingredients::ingredient_id.eq_any(ids))
    }

    pub fn recipe(&self, conn: &mut PgConnection) -> QueryResult<Recipe> {
        recipes::table.find(self.recipe_id).first::<Recipe>(conn)
    }

    pub fn ingredient(&self, conn: &mut PgConnection) -> QueryResult<Ingredient> {
        ingredients::table
            .find(self.ingredient_id)
            .first::<Ingredient>(conn)
    }

    pub fn measurement(
        &self,
        conn: &mut PgConnection,
        ingredient: &Ingredient,
    ) -> QueryResult<Measurement> {
        self.measurement_id
            .map(|id| measurements::table.find(id).first::<Measurement>(conn))
            .unwrap_or_else(|| {
                measurements::table
                    .find(ingredient.default_measurement_id)
                    .first::<Measurement>(conn)
            })
    }
}

impl<Db: Backend> All<Db> for RecipeIngredient {
    type Output = Select<recipe_ingredients::table, AsSelect<RecipeIngredient, Db>>;
    fn all() -> Self::Output {
        recipe_ingredients::table.select(Self::as_select())
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = recipe_ingredients)]
pub struct NewRecipeIngredient<'a> {
    pub recipe_id: RecipeId,
    pub ingredient_id: IngredientId,
    pub measurement_id: Option<MeasurementId>,
    pub quantity: BigDecimal,
    #[diesel(column_name = "idx")]
    pub index: i32,
    pub notes: Option<&'a str>,
}
