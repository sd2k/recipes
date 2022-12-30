use chrono::prelude::*;
use derive_more::{Display, From, Into};
use diesel::{
    backend::Backend,
    helper_types::{AsSelect, EqAny, Filter, Find, Select},
    prelude::*,
};
use diesel_derive_newtype::DieselNewType;

use crate::{
    schema::measurements,
    traits::{All, Findable},
};

#[derive(Clone, Copy, Debug, DieselNewType, Display, Eq, From, Hash, Into, PartialEq)]
pub struct MeasurementId(i64);

#[derive(Clone, Debug, Queryable, Identifiable, Selectable)]
pub struct Measurement {
    pub id: MeasurementId,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub abbreviation: Option<String>,
}

impl<Db: Backend> All<Db> for Measurement {
    type Output = Select<measurements::table, AsSelect<Measurement, Db>>;
    fn all() -> Self::Output {
        measurements::table.select(Self::as_select())
    }
}

impl<'a> Findable<'a> for Measurement {
    type Id = MeasurementId;
    type FindById = Find<measurements::table, Self::Id>;
    type FindByIds = Filter<measurements::table, EqAny<measurements::id, &'a [MeasurementId]>>;

    fn by_id(id: Self::Id) -> Self::FindById {
        measurements::table.find(id)
    }

    fn by_ids(ids: &'a [Self::Id]) -> Self::FindByIds {
        measurements::table.filter(measurements::id.eq_any(ids))
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = measurements)]
pub struct NewMeasurement<'a> {
    pub name: &'a str,
    pub abbreviation: Option<&'a str>,
}
