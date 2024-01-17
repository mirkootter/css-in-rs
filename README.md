# css-in-rs
A library for embedding dynamic CSS in Rust (wasm); inspired by [cssinjs/JSS](https://cssinjs.org/)

This crate is designed to be framework-independent; however, this requires a
little more work. For now, it only works with [Dioxus](https://dioxuslabs.com/)

## Use case
This crate allows to develop reusable components for the web which bundle their own
styles. Dead-Code-Analysis works as usually with Rust: If you do not need a certain component,
its styles won't be embedded in the final binary.

The classnames are generated at runtime to avoid collisions (i.e. `css-123`).

## Basic idea
The `make_styles!` procmacro allows you to write CSS-like style sheets directly in Rust. You can
use normal class names without worrying about collisions. Even if another component uses the same
name, it does not matter. The procmacro automatically determines the classnames from the style
and generates a helper class containing the real class names.

#### Example (Dioxus):
```rust
#![allow(non_snake_case)]

use css_in_rs::{Classes, EmptyTheme, make_styles, use_style_provider_root};
use dioxus::prelude::*;

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        red_text {
            color: "red",
            margin: "5px",
        },
        "button" {
            margin: "5px",
            padding: "5px",
            width: "10em",
        },
        "button.primary" {
            border: "2px solid red",
        },
        "button.disabled" { // Shows a rust warning: "field never read"
            disabled: true,
        },
    }
}

fn Demo(cx: Scope) -> Element {
    let classes: &MyClasses = MyClasses::use_style(cx);

    render! {
        div {
            class: &classes.red_text as &str,
            "This text is supposed to be red.",
        }
        button {
            class: &classes.primary as &str,
            "Click me",
        }
    }
}

fn App(cx: Scope) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("main").unwrap();
    
    use_style_provider_root(cx, &root, || EmptyTheme);

    cx.render(rsx! {
        Demo {}
    })
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
```