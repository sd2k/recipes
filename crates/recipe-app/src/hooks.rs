use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_query::prelude::*;

use recipe_shared::Recipe;

use crate::server;

pub fn use_init(cx: Scope) {
    use_init_query_client::<QueryValue, QueryError, QueryKeys>(cx);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueryKeys {
    Recipes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryValue {
    Recipes(Vec<Recipe>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum QueryError {
    Server(ServerFnError),
}

pub fn use_recipes(cx: Scope) -> &UseQuery<QueryValue, QueryError, QueryKeys> {
    use_query(
        cx,
        || vec![QueryKeys::Recipes],
        |_| async {
            server::recipes()
                .await
                .map(QueryValue::Recipes)
                .map_err(QueryError::Server)
                .into()
        },
    )
}
