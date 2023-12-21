use dioxus_fullstack::prelude::*;

use recipe_app::Route;

fn main() {
    tracing_wasm::set_as_global_default();
    LaunchBuilder::<FullstackRouterConfig<Route>>::router().launch()
}
