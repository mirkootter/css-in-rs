# css-in-rs
A library for embedding dynamic CSS in Rust (wasm); inspired by [cssinjs/jss](https://cssinjs.org/)

This crate is designed to be framework-independent.
It currently provides integrations for [Dioxus](https://dioxuslabs.com/), which is disabled by default.

## Use case
This crate allows to develop reusable components for the web which bundle their own
styles. Thanks to dead-code-analysis in Rust, only the styles which are actually used will be included
in the final wasm binary.

Features:
* A procmacro `make_styles!` to write css directly in Rust
* A runtime to inject the styles on a as-need basis. If styles are not used, they
  won't be included in the final binary
* Styles will only be mounted once, even if requested multiple times
* Dynamically created classnames to avoid collisions. You can choose common names
  like `active` for multiple components without problems
* Compile time checks: Rust will warn you if classnames defined in your styles are
  never used. If they don't exist, you'll get an error

## Basic idea
You embed your CSS code with classnames of your choice using the `make_styles!` procmacro.
It will generate a new rust `struct` for the final runtime-names of the used css classes.
For each css class in your style, a `classname: String` member will be available in the struct.
See the documentation for `make_styles!` for more details.

Styles generated this way can be mounted. On the first mount, classnames are generated at runtime
to avoid collisions and returned in the struct created by the procmacro. You can therefore access
the created classnames using the struct. Since the struct's type is generated at compile time,
the compiler will complain if you use undefined css classes and warn you about unused classes.

Styles are only mounted once. You can try to do it repeatedly, but it will be a no-op. You'll get
a reference to the classnames-struct which was created the first time. Therefore, you can reuse
your created styles in many components, and you do no not need to worry about mounting them too
many times.

#### Example (Dioxus):
```rust
#![allow(non_snake_case)]

use css_in_rs::{Classes, EmptyTheme, make_styles, use_style_provider_quickstart};
use dioxus::prelude::*;

// Will create a new struct `MyClasses` with three members:
//  `red_text`, `primary` and `disabled`
make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        red_text {          // defines a new css class: `red_text`
            color: "red",
            margin: "5px",
        },
        "button" {          // does not define any new css classes
            margin: "5px",
            padding: "5px",
            width: "10em",
        },
        "button.primary" {  // defines a new css class: `primary`
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
    use_style_provider_quickstart(cx, || EmptyTheme);

    cx.render(rsx! {
        Demo {}
    })
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
```
