use wasm_bindgen::JsCast;

use crate::Theme;

use super::Backend;

pub struct WebSysBackend {
    current_style: String,
    styles: web_sys::Element,
}

impl WebSysBackend {
    pub fn quickstart() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        Self::new_and_mount_in_root(&document)
    }

    pub fn new_and_mount_in_root(root: &web_sys::Node) -> Self {
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
            current_style: Default::default(),
        }
    }
}

impl<T: Theme> Backend<T> for WebSysBackend {
    fn replace_all(&mut self, css: String) {
        self.current_style = css;
        self.styles.set_text_content(Some(&self.current_style));
    }

    fn run_updater(&mut self, updater: super::UpdaterFn<T>, theme: &T, counter: &mut u64) {
        // TODO: There is probably a much faster way than to append this style this way
        (updater)(theme, &mut self.current_style, counter);
        self.styles.set_text_content(Some(&self.current_style));
    }
}
