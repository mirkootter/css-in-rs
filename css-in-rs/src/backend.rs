#[cfg(feature = "web-sys")]
pub mod web;

use crate::Theme;

pub type CssGeneratorFn<T> = fn(&T, &mut String, &mut u64) -> ();

/// css-in-rs is backend agnostic. The default backend is based on web_sys,
/// but other backends are possible (i.e. just insert css into a string, for
/// example for server side rendering).
pub trait Backend<T: Theme>: 'static {
    /// Replaces all styles managed by this backend by the given CSS string
    fn replace_all(&mut self, css: String);

    /// Runs a given css generator and add the generated styles. The `generator`
    /// function is expected to append new rules to the given `String`. It may
    /// be empty, in which case the new style is to be returned. Alternatively,
    /// the backend may choose to put in all existing rules, in which case the
    /// new rules are to be appended.
    fn run_css_generator(&mut self, generator: CssGeneratorFn<T>, theme: &T, counter: &mut u64);
}
