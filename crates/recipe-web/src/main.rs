use log::LevelFilter;

use recipe_app::{app, RootProps};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    let origin = Box::leak(Box::new(web_sys::window().unwrap().origin()));
    dioxus::web::launch_with_props(
        app,
        RootProps {
            origin: origin.as_str(),
            initial_state: None.into(),
        },
        |cfg| cfg.hydrate(true),
    );
}
