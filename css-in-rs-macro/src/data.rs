use std::collections::BTreeMap;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

use crate::output::{Output, ToOutput};

pub mod entry;
pub mod selector;
pub mod signature;

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
pub struct Style {
    pub signature: signature::Signature,
    pub rules: RuleList,
}

impl Style {
    pub fn get_classnames(&self) -> Vec<syn::Ident> {
        let mut classnames = Default::default();
        self.rules.collect_classnames(&mut classnames);

        let mut result = Vec::new();
        result.reserve_exact(classnames.len());
        for (classname, span) in classnames {
            let ident = syn::Ident::new(&classname, span);
            result.push(ident);
        }

        result
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

impl Parse for Style {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let signature = input.parse::<signature::Signature>()?;

        let content;
        syn::braced!(content in input);
        let rules = content.parse::<RuleList>()?;

        let style = Style { signature, rules };
        Ok(style)
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

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::data::{
        selector::{Part, Selector},
        Rule,
    };

    use super::Style;

    #[test]
    fn simple() {
        let input = quote! {
            (_theme: MyTheme) -> MyClasses {
                "div.red_text" {
                    color: "red",
                },
                "div.blue_text" {
                    color: "blue",
                },
                my_class {
                    background_color: "#ababab",
                    "12%": false,
                },
            }
        };

        let selector_to_str = |sel: &Selector| {
            use core::fmt::Write;
            let mut result = String::new();

            let parts = &sel.parts;

            for part in parts {
                match part {
                    Part::Raw(s) => {
                        write!(result, "raw'{}'", s)
                    }
                    Part::ClassName(s) => {
                        write!(result, "classname'{}'", s)
                    }
                }
                .unwrap();
            }

            result
        };

        let style = syn::parse2::<Style>(input).unwrap();
        let rules = style.rules.rules;
        let rules: Vec<&Rule> = rules.iter().collect();
        let selectors: Vec<&Selector> = rules.iter().map(|r| &r.selector).collect();

        assert_eq!(selectors.len(), 3);
        assert_eq!(selector_to_str(selectors[0]), "raw'div'classname'red_text'");
        assert_eq!(
            selector_to_str(selectors[1]),
            "raw'div'classname'blue_text'"
        );
        assert_eq!(selector_to_str(selectors[2]), "classname'my_class'");

        // Third rule
        {
            let rule = rules[2];
            assert_eq!(rule.entries.len(), 2);

            let mut entries = rule.entries.iter();

            let entry = entries.next().unwrap();
            assert_eq!(entry.property, "background-color");

            let entry = entries.next().unwrap();
            assert_eq!(entry.property, "12%");
        }
    }
}
