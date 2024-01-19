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

pub enum RuleBody {
    AtRule {
        children: Punctuated<Rule, syn::token::Comma>,
    },
    Normal {
        entries: Punctuated<entry::Entry, syn::token::Comma>,
    },
}

impl RuleBody {
    fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        match self {
            RuleBody::AtRule { children } => {
                for child in children {
                    child.collect_classnames(result);
                }
            }
            RuleBody::Normal { .. } => {}
        }
    }
}

pub struct Rule {
    pub header: header::Header,
    pub body: RuleBody,
}

impl Rule {
    fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        self.header.collect_classnames(result);
        self.body.collect_classnames(result);
    }
}

pub struct RuleList {
    pub rules: Punctuated<Rule, syn::token::Comma>,
}

impl RuleList {
    pub fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        for rule in &self.rules {
            rule.collect_classnames(result);
        }
    }
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let header = input.parse::<header::Header>()?;

        let content;
        syn::braced!(content in input);

        let body = match header.at_rule {
            true => {
                let children = content.parse_terminated(Rule::parse, Token![,])?;
                RuleBody::AtRule { children }
            }
            false => {
                let entries = content.parse_terminated(entry::Entry::parse, Token![,])?;
                RuleBody::Normal { entries }
            }
        };

        let rule = Rule { header, body };

        Ok(rule)
    }
}

impl Parse for RuleList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rules = input.parse_terminated(Rule::parse, Token![,])?;
        Ok(RuleList { rules })
    }
}

impl ToOutput for RuleBody {
    fn append(&self, result: &mut Output) {
        match self {
            RuleBody::AtRule { children } => {
                for child in children {
                    child.append(result);
                }
            }
            RuleBody::Normal { entries } => {
                for entry in entries {
                    entry.append(result);
                }
            }
        }
    }
}

impl ToOutput for Rule {
    fn append(&self, result: &mut Output) {
        self.header.append(result);
        result.format_str.push_str(" {{\n");
        self.body.append(result);
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
