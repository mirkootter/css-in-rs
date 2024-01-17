#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_root, Classes, EmptyTheme};
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

make_styles! {
    (_theme: EmptyTheme) -> RedClass {
        "div.text" {
            color: "red",
        },
    }
}

make_styles! {
    (_theme: EmptyTheme) -> BlueClass {
        "div.text" {
            color: "blue",
        },
    }
}

fn RedText(cx: Scope) -> Element {
    let classname = &RedClass::use_style(cx).text as &str;

    cx.render(rsx! {
        div {
            class: classname,
            "This text is supposed to be red!",
        }
    })
}

fn BlueText(cx: Scope) -> Element {
    let classname = &BlueClass::use_style(cx).text as &str;

    cx.render(rsx! {
        div {
            class: classname,
            "This text is supposed to be blue!",
        }
    })
}

fn App(cx: Scope) -> Element {
    let root = use_main_element(cx);
    use_style_provider_root(cx, root, || EmptyTheme);

    cx.render(rsx! {
        RedText {}
        BlueText {}
        RedText {}
        BlueText {}
    })
}
