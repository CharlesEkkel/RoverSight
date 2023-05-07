#![allow(non_snake_case)]
mod components;
mod services;

use components::roverpic::*;
use dioxus::prelude::*;

pub fn MyApp(cx: Scope) -> Element {
    cx.render(rsx!(RoverPic {}, RoverPic {},))
}
