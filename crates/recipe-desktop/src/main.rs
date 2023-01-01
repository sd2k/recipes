use recipe_app::{app, RootProps};

fn main() {
    dioxus::desktop::launch_with_props(app, RootProps::default(), |c| {
        c.with_window(|c| c.with_title("Recipes"))
    });
}
