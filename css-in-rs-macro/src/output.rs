use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Output {
    pub format_str: String,
    pub params: TokenStream,
    map: BTreeMap<String, u64>,
}

impl Output {
    pub fn new(classnames: &[syn::Ident]) -> Self {
        let mut map = BTreeMap::default();
        for ident in classnames {
            let idx = map.len() as u64;
            let old = map.insert(ident.to_string(), idx);
            assert!(old.is_none());
        }

        Self {
            format_str: Default::default(),
            params: Default::default(),
            map,
        }
    }

    pub fn push_str(&mut self, s: &str) {
        let s = s.replace('{', "{{");
        let s = s.replace('}', "}}");
        self.format_str.push_str(&s);
    }

    pub fn push_classname(&mut self, name: &str) {
        let id = *self.map.get(name).unwrap();

        self.format_str.push_str("css-{}");
        quote!(, start + #id).to_tokens(&mut self.params);
    }
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let format_str = &self.format_str;
        let params = &self.params;

        let ts = quote! {
            let _ = write!(css, #format_str #params);
        };
        ts.to_tokens(tokens);
    }
}

/// Essentially, the `make_styles` creates one huge `write!` command.
/// It consists of one format string and many params. Whenever we want
/// to add something to this output, the need to add it to both the
/// format string and to the param list.
///
/// That's what this trait is for: Types which implement this trait
/// can be appended to this output easily.
pub trait ToOutput {
    fn append(&self, result: &mut Output);
}
