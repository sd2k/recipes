use dioxus_desktop::{Config, WindowBuilder};
use dioxus_fullstack::prelude::*;
use log::LevelFilter;

use recipe_app::app;

fn main() {
    server_fn::set_server_url("http://127.0.0.1:8080");
    dioxus_logger::init(LevelFilter::Debug).expect("failed to init logger");
    let config = Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
        .with_window(WindowBuilder::new().with_title("Recipes"));
    LaunchBuilder::new(app).desktop_cfg(config).launch()
}
