//! A library for embedding dynamic CSS in Rust (wasm); inspired by [cssinjs/jss](https://cssinjs.org/)
//!
//! This crate is designed to be framework-independent.
//! It currently provides integrations for [Dioxus](https://dioxuslabs.com/), which is disabled by default.
//!
//! ## Use case
//! This crate allows to develop reusable components for the web which bundle their own
//! styles. Thanks to dead-code-analysis in Rust, only the styles which are actually used will be included
//! in the final wasm binary.
//!
//! Features:
//! * A procmacro [make_styles!] to write css directly in Rust
//! * A runtime to inject the styles on a as-need basis. If styles are not used, they
//!   won't be included in the final binary
//! * Styles will only be mounted once, even if requested multiple times
//! * Dynamically created classnames to avoid collisions. You can choose common names
//!   like `active` for multiple components without problems
//! * Compile time checks: Rust will warn you if classnames defined in your styles are
//!   never used. If they don't exist, you'll get an error
//!
//! ## Basic idea
//! You embed your CSS code with classnames of your choice using the [make_styles!] procmacro.
//! It will generate a new rust `struct` for the final runtime-names of the used css classes.
//! For each css class in your style, a `classname: String` member will be available in the struct.
//! See the documentation for [make_styles!] for more details.
//!
//! Styles generated this way can be mounted. On the first mount, classnames are generated at runtime
//! to avoid collisions and returned in the struct created by the procmacro. You can therefore access
//! the created classnames using the struct. Since the struct's type is generated at compile time,
//! the compiler will complain if you use undefined css classes and warn you about unused classes.
//!
//! Styles are only mounted once. You can try to do it repeatedly, but it will be a no-op. You'll get
//! a reference to the classnames-struct which was created the first time. Therefore, you can reuse
//! your created styles in many components, and you do no not need to worry about mounting them too
//! many times.
//!
//! #### Example (Dioxus):
//! ```no_run
//! # #[cfg(feature = "dioxus")] {
//! #![allow(non_snake_case)]
//!
//! use css_in_rs::{Classes, EmptyTheme, make_styles, use_style_provider_quickstart};
//! use dioxus::prelude::*;
//!
//! // Will create a new struct `MyClasses` with three members:
//! //  `red_text`, `primary` and `disabled`
//! make_styles! {
//!     (_theme: EmptyTheme) -> MyClasses {
//!         red_text {          // defines a new css class: `red_text`
//!             color: "red",
//!             margin: "5px",
//!         },
//!         "button" {          // does not define any new css classes
//!             margin: "5px",
//!             padding: "5px",
//!             width: "10em",
//!         },
//!         "button.primary" {  // defines a new css class: `primary`
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

/// A trait for themes: Themes contain shared data which can be
/// used in your styles.
///
/// For example, you can define color, paddings, and so on in a datatype.
/// Alternatively, you can use css variables which is probably even better.
/// However, themes are an easy way to precompute some things, for example
/// complex box-shadows. They can also be changed at runtime which will
/// update all styles which depend on this theme.
///
/// If you do not need theme support, you can use [EmptyTheme]  
pub trait Theme: Clone + 'static {
    fn fast_cmp(&self, other: &Self) -> bool;
}

/// An empty theme. Use if no theme support is needed.
///
/// If you do need themes (i.e. certain data which is shared
/// between all your styles), use a custom type and implement
/// the [Theme] trait.
#[derive(Clone, Copy)]
pub struct EmptyTheme;

impl Theme for EmptyTheme {
    fn fast_cmp(&self, _: &Self) -> bool {
        true
    }
}

/// This trait will be implemented by the classnames-struct generated
/// by the [make_styles!] macro. You probably won't implement it yourself
/// unless you need something very specific which the macro cannot handle.
///
/// Example
/// ```
/// # use css_in_rs::{Classes, EmptyTheme};
/// struct MyClasses {
///     active: String,
///     disabled: String,
/// }
///
/// impl Classes for MyClasses {
///     type Theme = EmptyTheme;
///
///     fn generate(_: &Self::Theme, css: &mut String, counter: &mut u64) {
///         use core::fmt::Write;
///         writeln!(css, "css-{counter} {{ background-color: transparent; }}").unwrap();
///         *counter += 1;
///         writeln!(css, "css-{counter} {{ background-color: #f0f0f0; }}").unwrap();
///         *counter += 1;
///     }
///
///     fn new(start: u64) -> Self {
///         MyClasses {
///             active: format!("css-{}", start),
///             disabled: format!("css-{}", start + 1),
///         }
///     }
/// }
/// ```
pub trait Classes: Sized + 'static {
    /// The [Theme] which this style depend on
    type Theme: Theme;

    /// Generate the CSS rules. Use the provided `counter` to obtain unique classnames and
    /// increment it accordingly. The content of the rules may depend on the given theme,
    /// but the classnames must be the same whenever this method is called. The classnames
    /// must only depend on the `counter`, because they are cached.
    ///
    /// It is important that the number of classes does not change either. The runtime will
    /// panic if a second call to `generate` returns a different counter than the first time.
    ///
    /// Usually, this method will introduce an arbitrary number `n` of the css classes and
    /// increment the counter by exactly `n`. This `n` is usually a fixed constant
    fn generate(theme: &Self::Theme, css: &mut String, counter: &mut u64);

    /// The styles generated in [Self::generate] use unreadable classnames. The struct implementing
    /// this type should provide the user a way to access those classnames. The `start`
    /// parameter is the same as the `counter` param used in [Self::generate], which is necessary
    /// because the dynamic classnames will depend on it.
    fn new(start: u64) -> Self;

    /// Mount this style and return a reference to the classnames (which are represented by
    /// `Self`).
    /// Note that the style will only be mounted once, even if you use this
    /// hook from multiple components or your components will be used multiple
    /// times. The classnames will be the same every time, as long as the
    /// same [StyleProvider] is used, which is taken from the current context.
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
