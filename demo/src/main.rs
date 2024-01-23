#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes};
use dioxus::prelude::*;
use mui_dioxus::theme::MuiTheme;

mod sample;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

make_styles! {
    (_theme: MuiTheme) -> CssClasses {
        "body" {
            background_color: "#e9e9e9",
        },
        "#main" {
            padding_top: "16px",
        }
    }
}

fn App(cx: Scope) -> Element {
    use_style_provider_quickstart(cx, MuiTheme::default);
    CssClasses::use_style(cx);

    render!(
        mui_dioxus_demopanel::Panel {
            source: include_str!("sample.rs"),
            sample::Sample {}
        }
    )
}
