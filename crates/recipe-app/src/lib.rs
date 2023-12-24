#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{Header, MealPlansPage, RecipesPage},
    hooks::use_init,
};

mod components;
mod hooks;
pub mod server;

pub fn Wrapper(cx: Scope) -> Element {
    use_init(cx);
    cx.render(rsx!(
        Header {}
        Outlet::<Route> {}
    ))
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        RecipesPage {},
        #[route("/plans")]
        MealPlansPage {},
}
