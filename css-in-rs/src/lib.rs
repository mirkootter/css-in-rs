//! A library for embedding dynamic CSS in Rust (wasm); inspired by [cssinjs/JSS](https://cssinjs.org/)
//!
//! This crate is designed to be framework-independent.
//! It currently provides integrations for [Dioxus](https://dioxuslabs.com/), which is disabled by default.
//!
//! ## Use case
//! This crate allows to develop reusable components for the web which bundle their own
//! styles. Dead-Code-Analysis works as usually with Rust: If you do not need a certain component,
//! its styles won't be embedded in the final binary.
//!
//! The classnames are generated at runtime to avoid collisions (i.e. `css-123`).
//!
//! ## Basic idea
//! The [make_styles!] procmacro allows you to write CSS-like style sheets directly in Rust. You can
//! use normal class names without worrying about collisions. Even if another component uses the same
//! name, it does not matter. The procmacro automatically determines the classnames from the style
//! and generates a helper class containing the real class names.
//!
//! #### Example (Dioxus):
//! ```no_run
//! # #[cfg(feature = "dioxus")] {
//! #![allow(non_snake_case)]
//!
//! use css_in_rs::{Classes, EmptyTheme, make_styles, use_style_provider_quickstart};
//! use dioxus::prelude::*;
//!
//! make_styles! {
//!     (_theme: EmptyTheme) -> MyClasses {
//!         red_text {
//!             color: "red",
//!             margin: "5px",
//!         },
//!         "button" {
//!             margin: "5px",
//!             padding: "5px",
//!             width: "10em",
//!         },
//!         "button.primary" {
//!             border: "2px solid red",
//!         },
//!         "button.disabled" { // Shows a rust warning: "field never read"
//!             disabled: true,
//!         },
//!     }
//! }
//!
//! fn Demo(cx: Scope) -> Element {
//!     let classes: &MyClasses = MyClasses::use_style(cx);
//!
//!     render! {
//!         div {
//!             class: &classes.red_text as &str,
//!             "This text is supposed to be red.",
//!         }
//!         button {
//!             class: &classes.primary as &str,
//!             "Click me",
//!         }
//!     }
//! }
//!
//! fn App(cx: Scope) -> Element {
//!     use_style_provider_quickstart(cx, || EmptyTheme);
//!
//!     cx.render(rsx! {
//!         Demo {}
//!     })
//! }
//!
//! fn main() {
//!     // launch the web app
//!     dioxus_web::launch(App);
//! }
//! # }
//! ```

#![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]

#[doc_cfg(feature = "dioxus")]
use dioxus::prelude::*;

mod style_provider;

pub mod backend;

pub use css_in_rs_macro::make_styles;
use doc_cfg::doc_cfg;
pub use style_provider::StyleProvider;

pub trait Theme: Clone + 'static {
    fn fast_cmp(&self, other: &Self) -> bool;
}

#[derive(Clone, Copy)]
pub struct EmptyTheme;

impl Theme for EmptyTheme {
    fn fast_cmp(&self, _: &Self) -> bool {
        true
    }
}

pub trait Classes: Sized + 'static {
    type Theme: Theme;
    fn generate(theme: &Self::Theme, css: &mut String, counter: &mut u64);
    fn new(start: u64) -> Self;

    #[doc_cfg(feature = "dioxus")]
    fn use_style(cx: &ScopeState) -> &Self {
        let provider = use_style_provider(cx);
        provider.use_styles(cx)
    }
}

/// Quickly sets up a StyleProvider in the global document. Styles will be attached
/// to `window.document.head`
#[doc_cfg(feature = "dioxus")]
pub fn use_style_provider_quickstart<'a, T: Theme>(
    cx: &'a ScopeState,
    make_theme: impl FnOnce() -> T,
) -> &'a StyleProvider<T> {
    let provider = cx.use_hook(|| StyleProvider::quickstart_web(make_theme()));
    use_context_provider(cx, || provider.clone())
}

#[doc_cfg(feature = "dioxus")]
pub fn use_style_provider<T: Theme>(cx: &ScopeState) -> &StyleProvider<T> {
    use_context(cx).unwrap()
}
