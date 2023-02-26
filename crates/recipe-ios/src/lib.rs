use dioxus::prelude::*;
use mobile_entry_point::mobile_entry_point;

#[mobile_entry_point]
fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "hello world!"
        }
    })
}
