#![allow(non_snake_case)]
mod components;
mod services;

use components::roverpic::*;
use dioxus::prelude::*;
use services::rover_service::*;

pub fn MyApp(cx: Scope) -> Element {
    let currRover = use_state(cx, || RoverName::Curiosity);

    let photos: &UseFuture<Vec<Photo>> = use_future!(cx, |(currRover,)| {
        let current = currRover.current().to_owned();
        async move {
            get_photos(*current).await.unwrap_or(vec![])
        }
    });

    cx.render(rsx! {
        div {
            class: "grid grid-cols-[1fr,4fr,1fr] grid-rows-[3rem,1fr] place-content-center h-screen bg-gray-800 text-white",
            div {
                class: "col-span-3 bg-gray-500 text-center",
                RoverSelectors { setRover: move |r| { currRover.set(r) } },
                h1 { "EYES ON MARS" }
            }
            div {
                class: "bg-gray-600",
                "Empty space"
            }
            match photos.value() {
                Some(photos) => rsx!{
                    div {
                        class: "flex w-full",
                        photos.iter().map(|pic| rsx!{
                            RoverPic { photo: pic }
                        })
                    }
                },
                None => rsx!{
                    Loader {}
                }
            }
            div {
                class: "bg-gray-700",
                "Filters"
            }
        }
    })
}

fn Loader(cx: Scope) -> Element {
    cx.render(rsx!(div {class: "lds-ripple place-self-center", div {}, div {}}))
}

#[inline_props]
fn RoverSelectors<F>(cx: Scope, setRover: F) -> Element
where
    F: Fn(RoverName) -> (),
{
    cx.render(rsx! {
        nav {
            a { onclick: move |_| setRover(RoverName::Curiosity), "Curiosity" },
            a { onclick: move |_| setRover(RoverName::Opportunity), "Opportunity" },
            a { onclick: move |_| setRover(RoverName::Spirit), "Spirit" },
        }
    })
}
