use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

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

pub struct Style {
    pub signature: signature::Signature,
    pub rules: RuleList,
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

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::data::selector::{Part, Selector};

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
                ident {},
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
        let selectors: Vec<&Selector> = rules.iter().map(|r| &r.selector).collect();

        assert_eq!(selectors.len(), 3);
        assert_eq!(selector_to_str(selectors[0]), "raw'div'classname'red_text'");
        assert_eq!(
            selector_to_str(selectors[1]),
            "raw'div'classname'blue_text'"
        );
        assert_eq!(selector_to_str(selectors[2]), "classname'ident'");
    }
}
