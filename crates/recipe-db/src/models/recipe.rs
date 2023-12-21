use chrono::prelude::*;
use derive_more::{Display, From, Into};
use diesel::{backend::Backend, helper_types::*, prelude::*};
use diesel_derive_newtype::DieselNewType;

use crate::{
    models::IngredientId,
    schema::{recipe_ingredients, recipes},
    traits::{All, Findable},
};

#[derive(Clone, Copy, Debug, DieselNewType, Display, Eq, From, Hash, Into, PartialEq)]
pub struct RecipeId(i64);

type FindBySlug<'a> = Filter<recipes::table, Eq<recipes::slug, &'a str>>;

type RecipeIngredientsBelongingToIngredient<'a> =
    Filter<recipe_ingredients::table, EqAny<recipe_ingredients::ingredient_id, &'a [IngredientId]>>;
// type RecipeIngredientsBelongingToIngredient<'a> =
// <RecipeIngredient as BelongingToDsl<&'a [Ingredient]>>::Output;
// type RecipeIngredientIdsBelongingToIngredient<'a> =
//     Select<RecipeIngredientsBelongingToIngredient<'a>, recipe_ingredients::recipe_id>;
// type FilteredByIngredient<'a> =
//     Filter<recipes::table, EqAny<recipes::id, RecipeIngredientIdsBelongingToIngredient<'a>>>;

type RecipeIngredientIdsBelongingToIngredient<'a> =
    Select<RecipeIngredientsBelongingToIngredient<'a>, recipe_ingredients::recipe_id>;
type FilteredByIngredientId<'a> =
    Filter<recipes::table, EqAny<recipes::id, RecipeIngredientIdsBelongingToIngredient<'a>>>;
type IngredientsForRecipe = Filter<
    recipe_ingredients::table,
    diesel::helper_types::Eq<recipe_ingredients::recipe_id, RecipeId>,
>;

#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
pub struct Recipe {
    pub id: RecipeId,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub source: Option<String>,
    pub notes: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cooking_time_minutes: Option<i32>,
    pub image_url: Option<String>,
}

impl Recipe {
    pub fn by_slug(slug: &str) -> FindBySlug {
        recipes::table.filter(recipes::slug.eq(slug))
    }

    pub fn by_ingredient_ids(ids: &[IngredientId]) -> FilteredByIngredientId {
        let recipe_ingredient_recipe_ids = recipe_ingredients::table
            .filter(recipe_ingredients::ingredient_id.eq_any(ids))
            .select(recipe_ingredients::recipe_id);
        recipes::table.filter(recipes::id.eq_any(recipe_ingredient_recipe_ids))
    }

    // pub fn by_ingredients(ingredients: &[Ingredient]) -> FilteredByIngredient {
    //     let recipe_ingredient_ids: RecipeIngredientIdsBelongingToIngredient =
    //         RecipeIngredient::belonging_to(ingredients).select(recipe_ingredients::recipe_id);
    //     recipes::table.filter(recipes::id.eq_any(recipe_ingredient_ids))
    // }

    pub fn ingredients(&self) -> IngredientsForRecipe {
        recipe_ingredients::table.filter(recipe_ingredients::recipe_id.eq(self.id))
    }
}

impl<Db: Backend> All<Db> for Recipe {
    type Output = Select<recipes::table, AsSelect<Recipe, Db>>;
    fn all() -> Self::Output {
        recipes::table.select(Self::as_select())
    }
}

impl<'a> Findable<'a> for Recipe {
    type Id = RecipeId;
    type FindById = Find<recipes::table, RecipeId>;
    type FindByIds = Filter<recipes::table, EqAny<recipes::id, &'a [RecipeId]>>;

    fn by_id(id: Self::Id) -> Self::FindById {
        recipes::table.find(id)
    }
    fn by_ids(ids: &'a [Self::Id]) -> Self::FindByIds {
        recipes::table.filter(recipes::id.eq_any(ids))
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = recipes)]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub source: Option<&'a str>,
    pub notes: Option<&'a str>,
    pub prep_time_minutes: Option<i32>,
    pub cooking_time_minutes: Option<i32>,
    pub image_url: Option<&'a str>,
}
