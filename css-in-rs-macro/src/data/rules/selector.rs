use std::collections::BTreeMap;

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

use crate::output::{Output, ToOutput};

use super::header::Header;

#[derive(Debug)]
pub struct Selector {
    pub header: Header,
}

impl Selector {
    pub fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        self.header.collect_classnames(result);
    }
}

impl Parse for Selector {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let header = input.parse()?;
        Ok(Self { header })
    }
}

impl ToOutput for Selector {
    fn append(&self, result: &mut Output) {
        self.header.append(result);
    }
}
