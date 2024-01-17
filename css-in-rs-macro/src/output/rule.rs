use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::LitStr;

use crate::data::Rule;

use super::ClassNameMap;

impl Rule {
    pub fn css_generator<'a>(&'a self, map: &'a ClassNameMap) -> RuleCssGenerator<'a> {
        RuleCssGenerator { map, rule: self }
    }
}

pub struct RuleCssGenerator<'a> {
    map: &'a ClassNameMap,
    rule: &'a Rule,
}

impl<'a> RuleCssGenerator<'a> {
    fn prepare_selector(&self) -> (String, TokenStream) {
        let mut format_str = String::new();
        let mut params = TokenStream::default();

        for part in &self.rule.selector.parts {
            match part {
                crate::data::selector::Part::Raw(raw) => {
                    format_str += raw;
                }
                crate::data::selector::Part::ClassName(classname) => {
                    let id = self.map.get(classname).unwrap().1;
                    format_str += ".css-{}";

                    let param = quote!(, start + #id);
                    param.to_tokens(&mut params);
                }
            }
        }

        (format_str, params)
    }
}

impl<'a> ToTokens for RuleCssGenerator<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (mut format_str, params) = self.prepare_selector();

        let entries = self.rule.entries.iter();

        format_str += " {{";
        let format_str = LitStr::new(&format_str, Span::call_site());
        let write_sel = quote! {
            ::core::writeln!(css, #format_str #params).unwrap();
            #(#entries)*
            ::core::writeln!(css, "}}").unwrap();
        };
        write_sel.to_tokens(tokens);
    }
}
