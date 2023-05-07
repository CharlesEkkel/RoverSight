#![allow(non_snake_case)]

use dioxus::prelude::*;
use roverpics::MyApp;

fn main() {
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!(MyApp {},))
}
