use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::Route;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        header { class: "navbar bg-neutral text-neutral-content",
            div { class: "navbar-start",
                div { class: "dropdown",
                    div { class: "btn btn-ghost lg:hidden", tabindex: "0", role: "button",
                        svg {
                            class: "h-5 w-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6h16M4 12h8m-8 6h16"
                            }
                        }
                    }
                    ul {
                        class: "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52",
                        tabindex: "0",
                        li { a { class: "mr-5 hover:text-white", "Recipes" } }
                        li { a { class: "mr-5 hover:text-white", "Meal plans" } }
                    }
                }
                a { class: "btn btn-ghost text-xl", "Recipe organiser" }
            }
            div { class: "navbar-center hidden lg:flex",
                ul { class: "menu menu-horizontal px-1",
                    li { Link { to: Route::RecipesPage {}, class: "mr-5 hover:text-white", "Recipes" } }
                    li { Link { to: Route::MealPlansPage {}, class: "mr-5 hover:text-white", "Meal plans" } }
                }
            }
            div { class: "navbar-end",
                div { class: "form-control", input {
                    r#type: "text",
                    placeholder: "Search",
                    class: "input input-bordered w-24 md:w-auto"
                } }
            }
        }
    ))
}
