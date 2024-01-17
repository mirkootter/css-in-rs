#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn use_main_element(cx: &ScopeState) -> &web_sys::Element {
    cx.use_hook(|| {
        let doc = web_sys::window().unwrap().document().unwrap();
        doc.get_element_by_id("main").unwrap()
    })
}

fn App(cx: Scope) -> Element {
    let _root = use_main_element(cx);

    cx.render(rsx! {
        "This is just a stub. More to come",
    })
}
