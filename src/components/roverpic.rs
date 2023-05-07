use dioxus::prelude::*;

pub fn RoverPic(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            "This is a mars rover picture"
        }
    ))
}
