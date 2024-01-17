use std::collections::BTreeMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use crate::data::Style;

pub mod rule;

type ClassNameMap = BTreeMap<String, (Span, u64)>;

pub struct ClassDefinition<'a> {
    style: &'a Style,
}

pub struct TraitImpl<'a> {
    style: &'a Style,
}

impl Style {
    pub fn class_definition(&self) -> ClassDefinition {
        ClassDefinition { style: self }
    }

    pub fn trait_impl(&self) -> TraitImpl {
        TraitImpl { style: self }
    }
}

impl<'a> ToTokens for ClassDefinition<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut decls = TokenStream::default();

        let classnames = self.style.get_classnames();
        decls.append_all(classnames.into_iter().map(|ident| {
            quote_spanned!(ident.span() =>
                pub #ident: String,
            )
        }));

        let classname = &self.style.signature.classname;
        let header = quote_spanned!(classname.span() => struct #classname);
        let result = quote!(
            #header {
                #decls
            }
        );

        result.to_tokens(tokens)
    }
}

impl<'a> ToTokens for TraitImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let classname = &self.style.signature.classname;
        let theme_var = &self.style.signature.theme_varname;
        let theme_type = &self.style.signature.theme_type;

        let map = {
            let mut map = BTreeMap::default();
            let classes = self.style.get_classnames();
            for ident in classes {
                let id = map.len() as u64;
                map.insert(ident.to_string(), (ident.span(), id));
            }
            map
        };

        let number_of_classes = map.len() as u64;
        let rules = self
            .style
            .rules
            .rules
            .iter()
            .map(|rule| rule.css_generator(&map));

        let setup_classnames = map.iter().map(|(name, (span, id))| {
            let ident = Ident::new(name, span.clone());
            quote_spanned! {span.clone() =>
                #ident: ::std::format!("css-{}", start + #id),
            }
        });

        let result = quote! {
            impl ::css_in_rs::Classes for #classname {
                type Theme = #theme_type;

                fn generate(#theme_var: &Self::Theme, css: &mut String, counter: &mut u64) {
                    use ::core::fmt::Write;
                    let start = *counter;
                    #(#rules)*
                    *counter = start + #number_of_classes;
                }

                fn new(start: u64) -> Self {
                    Self {
                        #(#setup_classnames)*
                    }
                }
            }
        };

        result.to_tokens(tokens)
    }
}
