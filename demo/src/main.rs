#![allow(non_snake_case)]

use css_in_rs::{use_style_provider, use_style_provider_root, EmptyTheme};
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

fn RedText(cx: Scope) -> Element {
    let provider = use_style_provider::<EmptyTheme>(cx);
    let classname = cx.use_hook(|| {
        let start = provider.add_updater(|_, css, counter| {
            use core::fmt::Write;
            writeln!(css, ".css-{counter} {{ color: red; }}").unwrap();
            *counter += 1;
        });

        format!("css-{start}")
    }) as &str;

    render!(
        div {
            class: classname,
            "This text is supposed to be red!",
        }
    )
}

fn BlueText(cx: Scope) -> Element {
    let provider = use_style_provider::<EmptyTheme>(cx);
    let classname = cx.use_hook(|| {
        let start = provider.add_updater(|_, css, counter| {
            use core::fmt::Write;
            writeln!(css, ".css-{counter} {{ color: blue; }}").unwrap();
            *counter += 1;
        });

        format!("css-{start}")
    }) as &str;

    render!(
        div {
            class: classname,
            "This text is supposed to be blue!",
        }
    )
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
