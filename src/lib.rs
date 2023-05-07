#![allow(non_snake_case)]
mod components;
mod services;

use components::roverpic::*;
use dioxus::prelude::*;
use services::rover_service::*;

pub fn MyApp(cx: Scope) -> Element {

    let photos: &UseFuture<Vec<Photo>> = use_future(cx, (), |_| async move {
        get_photos().await.unwrap_or(vec![])
    });

    cx.render(match photos.value() {
        Some(photos) => rsx!{
            photos.iter().map(|pic| cx.render(rsx!{
                RoverPic { photo: pic }
            }))
        },
        None => rsx!{
            div {"Whoops"}
        }
    })
}
