use dioxus_desktop::{Config, WindowBuilder};
use recipe_app::{app, RootProps};

fn main() {
    dioxus_desktop::launch_with_props(
        app,
        RootProps::default(),
        Config::new().with_window(WindowBuilder::new().with_title("Recipes")),
    );
}
