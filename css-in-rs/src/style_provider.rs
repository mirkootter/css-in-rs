use core::cell::RefCell;
use std::{collections::btree_map::Entry, rc::Rc};

use dioxus::core::ScopeState;
use wasm_bindgen::JsCast;

use crate::{Classes, Theme};

#[derive(Clone)]
pub struct StyleProvider<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T: Theme> StyleProvider<T> {
    pub fn new_and_mount(some_elem: &web_sys::Element, theme: T) -> Self {
        let inner = Inner::new_and_mount(some_elem, theme);
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

    pub fn use_styles<'a, C>(&self, cx: &'a ScopeState) -> &'a C
    where
        C: Classes<Theme = T>,
    {
        cx.use_hook(|| self.add_classes())
    }
}

type UpdaterFn<T> = fn(&T, &mut String, &mut u64) -> ();

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
    styles: web_sys::Element,
    current_theme: T,
    current_style: String,
    updaters: Vec<Updater<T>>,
    updater_to_idx: std::collections::BTreeMap<UpdaterFn<T>, usize>,
    counter: u64,
}

impl<T: Theme> Inner<T> {
    pub fn new_and_mount(some_elem: &web_sys::Element, theme: T) -> Self {
        let root = some_elem.get_root_node();

        let styles = if let Some(doc) = root.dyn_ref::<web_sys::Document>() {
            let head = doc.head().unwrap();
            let styles = doc.create_element("style").unwrap();
            head.append_child(&styles).unwrap();
            styles
        } else {
            panic!("This is most likely a shadow root. Not supported yet");
        };

        Self {
            styles,
            current_theme: theme,
            current_style: Default::default(),
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
        updater(
            &self.current_theme,
            &mut self.current_style,
            &mut self.counter,
        );
        let stop = self.counter;
        let updater = Updater {
            updater,
            start,
            stop,
        };

        self.updaters.push(updater);

        // TODO: Probably much faster just to add a single CSS Rule
        self.styles.set_text_content(Some(&self.current_style));

        start
    }

    fn update(&mut self) {
        self.current_style.clear();
        for updater in &self.updaters {
            updater.update(&self.current_theme, &mut self.current_style);
        }

        self.styles.set_text_content(Some(&self.current_style));
    }

    pub fn update_theme(&mut self, theme: T) {
        if !self.current_theme.fast_cmp(&theme) {
            self.current_theme = theme;
            self.update();
        }
    }
}
