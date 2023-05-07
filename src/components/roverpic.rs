use dioxus::prelude::*;

use crate::services::rover_service::Photo;

#[inline_props]
pub fn RoverPic<'a>(cx: Scope, photo: &'a Photo) -> Element {
    cx.render(rsx! {img { src: "{photo.img_src}" }})
}
