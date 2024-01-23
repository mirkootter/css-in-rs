#[cfg(feature = "web-sys")]
pub mod web;

use crate::Theme;

pub type CssGeneratorFn<T> = fn(&T, &mut String, &mut u64) -> ();

pub trait Backend<T: Theme>: 'static {
    /// Replaces all styles managed by this backend by the given CSS string
    fn replace_all(&mut self, css: String);

    fn run_css_generator(&mut self, generator: CssGeneratorFn<T>, theme: &T, counter: &mut u64);
}
