use chrono::prelude::*;
use derive_more::{Display, From, Into};
use diesel::{
    backend::Backend,
    helper_types::{AsSelect, EqAny, Filter, Find, Select},
    prelude::*,
};
use diesel_derive_newtype::DieselNewType;

use crate::{
    models::MeasurementId,
    schema::ingredients,
    traits::{All, Findable},
};

#[derive(Clone, Copy, Debug, DieselNewType, Display, Eq, From, Hash, Into, PartialEq)]
pub struct IngredientId(i64);

#[derive(Clone, Debug, Queryable, Identifiable, AsChangeset, Selectable)]
pub struct Ingredient {
    pub id: IngredientId,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub default_measurement_id: MeasurementId,
}

impl<Db: Backend> All<Db> for Ingredient {
    type Output = Select<ingredients::table, AsSelect<Ingredient, Db>>;
    fn all() -> Self::Output {
        ingredients::table.select(Self::as_select())
    }
}

impl<'a> Findable<'a> for Ingredient {
    type Id = IngredientId;
    type FindById = Find<ingredients::table, Self::Id>;
    type FindByIds = Filter<ingredients::table, EqAny<ingredients::id, &'a [IngredientId]>>;

    fn by_id(id: Self::Id) -> Self::FindById {
        ingredients::table.find(id)
    }

    fn by_ids(ids: &'a [Self::Id]) -> Self::FindByIds {
        ingredients::table.filter(ingredients::id.eq_any(ids))
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = ingredients)]
pub struct NewIngredient<'a> {
    pub name: &'a str,
    pub default_measurement_id: MeasurementId,
}
