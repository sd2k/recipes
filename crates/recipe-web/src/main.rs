use dioxus_fullstack::prelude::*;

use recipe_app::app;

fn main() {
    tracing_wasm::set_as_global_default();
    LaunchBuilder::new(app).launch()
}
