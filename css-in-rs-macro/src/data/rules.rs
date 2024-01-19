use std::collections::BTreeMap;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

use crate::output::{Output, ToOutput};

pub mod entry;
pub mod header;
pub mod selector;

pub struct Rule {
    pub selector: selector::Selector,
    pub entries: Punctuated<entry::Entry, syn::token::Comma>,
}

pub struct RuleList {
    pub rules: Punctuated<Rule, syn::token::Comma>,
}

impl RuleList {
    pub fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        for rule in &self.rules {
            rule.selector.collect_classnames(result);
        }
    }
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let selector = input.parse::<selector::Selector>()?;

        let content;
        syn::braced!(content in input);

        let entries = content.parse_terminated(entry::Entry::parse, Token![,])?;

        let rule = Rule { selector, entries };

        Ok(rule)
    }
}

impl Parse for RuleList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rules = input.parse_terminated(Rule::parse, Token![,])?;
        Ok(RuleList { rules })
    }
}

impl ToOutput for Rule {
    fn append(&self, result: &mut Output) {
        self.selector.append(result);
        result.format_str.push_str(" {{\n");
        for entry in &self.entries {
            entry.append(result);
        }
        result.format_str.push_str("}}\n");
    }
}

impl ToOutput for RuleList {
    fn append(&self, result: &mut Output) {
        for rule in &self.rules {
            rule.append(result);
        }
    }
}
