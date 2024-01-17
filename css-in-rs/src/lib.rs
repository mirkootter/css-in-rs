use dioxus::prelude::*;

mod style_provider;

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

    fn use_style(cx: &ScopeState) -> &Self {
        let provider = use_style_provider(cx);
        provider.use_styles(cx)
    }
}

pub fn use_style_provider_root<'a, T: Theme>(
    cx: &'a ScopeState,
    some_elem: &web_sys::Element,
    make_theme: impl FnOnce() -> T,
) -> &'a StyleProvider<T> {
    let provider = cx.use_hook(|| StyleProvider::new_and_mount(some_elem, make_theme()));
    use_context_provider(cx, || provider.clone())
}

pub fn use_style_provider<T: Theme>(cx: &ScopeState) -> &StyleProvider<T> {
    use_context(cx).unwrap()
}
