use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

pub mod selector;

pub struct Rule {
    pub selector: selector::Selector,
}

pub struct RuleList {
    pub rules: Punctuated<Rule, syn::token::Comma>,
}

pub struct Style {
    pub rules: RuleList,
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let selector = input.parse::<selector::Selector>()?;

        let _content;
        syn::braced!(_content in input);

        let rule = Rule { selector };

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
        let rules = input.parse::<RuleList>()?;

        let style = Style { rules };
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
            "div.red_text" {},
            "div.blue_text" {},
            ident {},
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
