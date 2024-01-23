use core::cell::RefCell;
use std::{collections::btree_map::Entry, rc::Rc};

#[doc_cfg(feature = "dioxus")]
use dioxus::core::ScopeState;

use doc_cfg::doc_cfg;

use crate::{
    backend::{Backend, CssGeneratorFn},
    Classes, Theme,
};

/// Manages dynamically inserted styles. You should usually have exactly one.
/// Generated classnames are only unique for a fixed [StyleProvider].
///
/// You will typically use [StyleProvider] in comination with the [`crate::make_styles!`]
/// macro.
///
/// # Example
/// ```no_run
/// # use css_in_rs::{make_styles, EmptyTheme, StyleProvider};
/// make_styles! {
///     (_theme: EmptyTheme) -> MyClasses {
///         ".my_class > span" {
///             color: "red",
///         },
///     }
/// }
///
/// fn main() {
///     let style_provider = StyleProvider::quickstart_web(EmptyTheme);
///     
///     // inject the css styles
///     let cls = style_provider.add_classes::<MyClasses>();
///     let elem: &web_sys::Element = todo!(); // Some element
///     elem.set_class_name(&cls.my_class);
///     
///     // inject it again; no change; will return the same classes
///     let cls2 = style_provider.add_classes::<MyClasses>();
///     assert_eq!(cls.my_class, cls2.my_class);
/// }
/// ```
#[derive(Clone)]
pub struct StyleProvider<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T: Theme> StyleProvider<T> {
    /// Quickly sets up a [StyleProvider] for the given theme at the active document.
    /// It will create a new `style` tag to the head and use it to mount new styles.
    #[cfg(feature = "web-sys")]
    pub fn quickstart_web(theme: T) -> Self {
        let inner = Inner::quickstart_web(theme);
        let inner = Rc::new(RefCell::new(inner));

        StyleProvider { inner }
    }

    fn add_css_generator(&self, generator: CssGeneratorFn<T>) -> u64 {
        self.inner.borrow_mut().add_css_generator(generator)
    }

    /// Mount new styles and returns the dynamically generated classnames.
    /// If this style is already mounted, it won't be mounted again. The classnames
    /// will be the same as last time.
    pub fn add_classes<C>(&self) -> C
    where
        C: Classes<Theme = T>,
    {
        let start = self.add_css_generator(C::generate);
        C::new(start)
    }

    /// Change the theme. All styles will be recomputed, but the classnames will
    /// not change.
    pub fn update_theme(&self, theme: T) {
        self.inner.borrow_mut().update_theme(theme);
    }

    /// A convenience hook to mount styles and cache the classnames.
    /// Note that the style will only be mounted once, even if you use this
    /// hook from multiple components or your components will be used multiple
    /// times. The classnames will be the same every time, as long as the
    /// same [StyleProvider] is used.
    #[doc_cfg(feature = "dioxus")]
    pub fn use_styles<'a, C>(&self, cx: &'a ScopeState) -> &'a C
    where
        C: Classes<Theme = T>,
    {
        cx.use_hook(|| self.add_classes())
    }
}

struct CssGenerator<T> {
    generator: CssGeneratorFn<T>,
    start: u64,
    stop: u64,
}

impl<T: Theme> CssGenerator<T> {
    fn generate(&self, theme: &T, css: &mut String) {
        let mut counter = self.start;
        (self.generator)(theme, css, &mut counter);
        assert_eq!(counter, self.stop);
    }
}

struct Inner<T> {
    backend: Box<dyn Backend<T>>,
    current_theme: T,
    generators: Vec<CssGenerator<T>>,
    generator_to_idx: std::collections::BTreeMap<CssGeneratorFn<T>, usize>,
    counter: u64,
}

impl<T: Theme> Inner<T> {
    #[cfg(feature = "web-sys")]
    pub fn quickstart_web(theme: T) -> Self {
        let backend = crate::backend::web::WebSysBackend::quickstart();
        Self::new_with_backend(backend, theme)
    }

    pub fn new_with_backend<B: Backend<T>>(backend: B, theme: T) -> Self {
        let backend = Box::new(backend);
        Self {
            backend,
            current_theme: theme,
            generators: Default::default(),
            generator_to_idx: Default::default(),
            counter: 0,
        }
    }

    pub fn add_css_generator(&mut self, generator: CssGeneratorFn<T>) -> u64 {
        debug_assert_eq!(self.generator_to_idx.len(), self.generators.len());

        match self.generator_to_idx.entry(generator) {
            Entry::Vacant(vac) => {
                vac.insert(self.generators.len());
            }
            Entry::Occupied(occ) => {
                let idx = *occ.get();
                return self.generators[idx].start;
            }
        }

        let start = self.counter;
        self.backend
            .run_css_generator(generator, &self.current_theme, &mut self.counter);
        let stop = self.counter;
        let generator = CssGenerator {
            generator,
            start,
            stop,
        };

        self.generators.push(generator);
        start
    }

    fn update(&mut self) {
        let mut css = String::default();
        for generator in &self.generators {
            generator.generate(&self.current_theme, &mut css);
        }

        self.backend.replace_all(css);
    }

    pub fn update_theme(&mut self, theme: T) {
        if !self.current_theme.fast_cmp(&theme) {
            self.current_theme = theme;
            self.update();
        }
    }
}
