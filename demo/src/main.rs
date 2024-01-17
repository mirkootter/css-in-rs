#![allow(non_snake_case)]

use css_in_rs::{use_style_provider_root, Classes, EmptyTheme};
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

struct RedClass(String);
struct BlueClass(String);

impl Classes for RedClass {
    type Theme = EmptyTheme;

    fn generate(_theme: &Self::Theme, css: &mut String, counter: &mut u64) {
        use core::fmt::Write;
        writeln!(css, ".css-{counter} {{ color: red; }}").unwrap();
        *counter += 1;
    }

    fn new(start: u64) -> Self {
        Self(format!("css-{start}"))
    }
}

impl Classes for BlueClass {
    type Theme = EmptyTheme;

    fn generate(_theme: &Self::Theme, css: &mut String, counter: &mut u64) {
        use core::fmt::Write;
        writeln!(css, ".css-{counter} {{ color: blue; }}").unwrap();
        *counter += 1;
    }

    fn new(start: u64) -> Self {
        Self(format!("css-{start}"))
    }
}

fn RedText(cx: Scope) -> Element {
    let classname = &RedClass::use_style(cx).0 as &str;

    cx.render(rsx! {
        div {
            class: classname,
            "This text is supposed to be red!",
        }
    })
}

fn BlueText(cx: Scope) -> Element {
    let classname = &BlueClass::use_style(cx).0 as &str;

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
