#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

make_styles! {
    (_theme: EmptyTheme) -> SharedClasses {
        left_margin {
            margin_left: "16px",
        },
    }
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
    let class1 = &SharedClasses::use_style(cx).left_margin;
    let class2 = &RedClass::use_style(cx).text;

    cx.render(rsx! {
        div {
            class: "{class1} {class2}",
            "This text is supposed to be red!",
        }
    })
}

fn BlueText(cx: Scope) -> Element {
    let class1 = &SharedClasses::use_style(cx).left_margin;
    let class2 = &BlueClass::use_style(cx).text;

    cx.render(rsx! {
        div {
            class: "{class1} {class2}",
            "This text is supposed to be blue!",
        }
    })
}

fn App(cx: Scope) -> Element {
    use_style_provider_quickstart(cx, || EmptyTheme);

    cx.render(rsx! {
        RedText {}
        BlueText {}
        RedText {}
        BlueText {}
    })
}
