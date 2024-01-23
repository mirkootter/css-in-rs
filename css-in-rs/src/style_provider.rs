use core::cell::RefCell;
use std::{collections::btree_map::Entry, rc::Rc};

#[doc_cfg(feature = "dioxus")]
use dioxus::core::ScopeState;

use doc_cfg::doc_cfg;

use crate::{
    backend::{self, Backend, UpdaterFn},
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
    pub fn quickstart_web(theme: T) -> Self {
        let inner = Inner::quickstart_web(theme);
        let inner = Rc::new(RefCell::new(inner));

        StyleProvider { inner }
    }

    fn add_updater(&self, updater: fn(&T, &mut String, &mut u64)) -> u64 {
        self.inner.borrow_mut().add_updater(updater)
    }

    pub fn add_classes<C>(&self) -> C
    where
        C: Classes<Theme = T>,
    {
        let start = self.add_updater(C::generate);
        C::new(start)
    }

    pub fn update_theme(&self, theme: T) {
        self.inner.borrow_mut().update_theme(theme);
    }

    #[doc_cfg(feature = "dioxus")]
    pub fn use_styles<'a, C>(&self, cx: &'a ScopeState) -> &'a C
    where
        C: Classes<Theme = T>,
    {
        cx.use_hook(|| self.add_classes())
    }
}

struct Updater<T> {
    updater: UpdaterFn<T>,
    start: u64,
    stop: u64,
}

impl<T: Theme> Updater<T> {
    fn update(&self, theme: &T, css: &mut String) {
        let mut counter = self.start;
        (self.updater)(theme, css, &mut counter);
        assert_eq!(counter, self.stop);
    }
}

struct Inner<T> {
    backend: Box<dyn Backend<T>>,
    current_theme: T,
    updaters: Vec<Updater<T>>,
    updater_to_idx: std::collections::BTreeMap<UpdaterFn<T>, usize>,
    counter: u64,
}

impl<T: Theme> Inner<T> {
    pub fn quickstart_web(theme: T) -> Self {
        let backend = backend::web::WebSysBackend::quickstart();
        Self::new_with_backend(backend, theme)
    }

    pub fn new_with_backend<B: Backend<T>>(backend: B, theme: T) -> Self {
        let backend = Box::new(backend);
        Self {
            backend,
            current_theme: theme,
            updaters: Default::default(),
            updater_to_idx: Default::default(),
            counter: 0,
        }
    }

    pub fn add_updater(&mut self, updater: UpdaterFn<T>) -> u64 {
        debug_assert_eq!(self.updater_to_idx.len(), self.updaters.len());

        match self.updater_to_idx.entry(updater) {
            Entry::Vacant(vac) => {
                vac.insert(self.updaters.len());
            }
            Entry::Occupied(occ) => {
                let idx = *occ.get();
                return self.updaters[idx].start;
            }
        }

        let start = self.counter;
        self.backend
            .run_updater(updater, &self.current_theme, &mut self.counter);
        let stop = self.counter;
        let updater = Updater {
            updater,
            start,
            stop,
        };

        self.updaters.push(updater);
        start
    }

    fn update(&mut self) {
        let mut css = String::default();
        for updater in &self.updaters {
            updater.update(&self.current_theme, &mut css);
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
