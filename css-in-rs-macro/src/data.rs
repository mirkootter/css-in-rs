use syn::parse::{Parse, ParseStream};

pub mod rules;
pub mod signature;

pub struct Style {
    pub signature: signature::Signature,
    pub rules: rules::RuleList,
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

impl Parse for Style {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let signature = input.parse::<signature::Signature>()?;

        let content;
        syn::braced!(content in input);
        let rules = content.parse::<rules::RuleList>()?;

        let style = Style { signature, rules };
        Ok(style)
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::data::rules::{
        header::{Header, Part},
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

        let header_to_str = |header: &Header| {
            use core::fmt::Write;
            let mut result = String::new();

            let parts = &header.parts;

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
        let headers: Vec<&Header> = rules.iter().map(|r| &r.header).collect();

        assert_eq!(headers.len(), 3);
        assert_eq!(header_to_str(headers[0]), "raw'div.'classname'red_text'");
        assert_eq!(header_to_str(headers[1]), "raw'div.'classname'blue_text'");
        assert_eq!(header_to_str(headers[2]), "raw'.'classname'my_class'");

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
