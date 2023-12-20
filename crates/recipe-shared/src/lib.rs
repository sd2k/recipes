use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Recipe {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub source: Option<String>,
    pub notes: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cooking_time_minutes: Option<i32>,
    pub image_url: Option<String>,
}
