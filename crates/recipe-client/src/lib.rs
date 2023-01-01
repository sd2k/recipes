#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Clone, Debug, PartialEq)]
    #[cynic(graphql_type = "QueryRoot")]
    pub struct AllRecipes {
        pub recipes: Vec<Recipe>,
    }

    #[derive(cynic::QueryFragment, Clone, Debug, PartialEq)]
    pub struct Recipe {
        pub id: cynic::Id,
        pub name: String,
        pub source: Option<String>,
        pub notes: Option<String>,
        pub ingredients: Vec<RecipeIngredient>,
        pub prep_time_minutes: Option<i32>,
        pub cooking_time_minutes: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Clone, Debug, PartialEq)]
    pub struct RecipeIngredient {
        pub quantity: Option<f64>,
        pub measurement: Option<Measurement>,
    }

    #[derive(cynic::QueryFragment, Clone, Debug, PartialEq)]
    pub struct Measurement {
        pub abbreviation: Option<String>,
    }
}

#[allow(non_snake_case, non_camel_case_types)]
mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}

pub mod prelude {
    pub use cynic::{
        http::{CynicReqwestError, ReqwestExt},
        MutationBuilder, QueryBuilder,
    };

    pub use crate::queries::*;
}

#[cfg(test)]
mod tests {
    use cynic::QueryBuilder;

    use super::queries::*;

    #[test]
    fn all_recipes_query_gql_output() {
        let operation = AllRecipes::build(());
        insta::assert_snapshot!(operation.query);
    }
}
