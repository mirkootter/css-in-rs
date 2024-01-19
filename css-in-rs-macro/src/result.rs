use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use crate::{
    data::{signature::Signature, Style},
    output::{Output, ToOutput},
};

struct ClassDefinition<'a> {
    classnames: &'a [syn::Ident],
    classname: &'a syn::Ident,
}

impl<'a> ToTokens for ClassDefinition<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut decls = TokenStream::default();

        decls.append_all(self.classnames.iter().map(|ident| {
            quote_spanned!(ident.span() =>
                pub #ident: String,
            )
        }));

        let classname = self.classname;
        let header = quote_spanned!(classname.span() => struct #classname);
        let result = quote!(
            #header {
                #decls
            }
        );

        result.to_tokens(tokens)
    }
}

struct TraitImpl<'a> {
    signature: &'a Signature,
    classnames: &'a [syn::Ident],
    output: &'a Output,
}

impl<'a> ToTokens for TraitImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let classname = &self.signature.classname;
        let theme_var = &self.signature.theme_varname;
        let theme_type = &self.signature.theme_type;
        let output = self.output;

        let number_of_classes = self.classnames.len() as u64;
        let setup_classnames =
            self.classnames
                .iter()
                .enumerate()
                .map(|(idx, ident)| -> TokenStream {
                    let idx = idx as u64;
                    quote! {
                        #ident: format!("css-{}", start + #idx),
                    }
                });

        let ts = quote! {
            impl ::css_in_rs::Classes for #classname {
                type Theme = #theme_type;

                fn generate(#theme_var: &Self::Theme, css: &mut String, counter: &mut u64) {
                    use ::core::fmt::Write;
                    let start = *counter;
                    #output
                    *counter = start + #number_of_classes;
                }

                fn new(start: u64) -> Self {
                    Self {
                        #(#setup_classnames)*
                    }
                }
            }
        };
        ts.to_tokens(tokens)
    }
}

pub struct Result {
    style: Style,
    classnames: Vec<syn::Ident>,
    output: Output,
}

impl Result {
    pub fn new(style: Style) -> Self {
        let classnames = style.get_classnames();
        let mut output = Output::new(&classnames);

        style.rules.append(&mut output);

        Self {
            style,
            classnames,
            output,
        }
    }
}

impl ToTokens for Result {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let signature = &self.style.signature;
        let classnames = &self.classnames;
        let class_def = ClassDefinition {
            classnames,
            classname: &signature.classname,
        };

        let trait_impl = TraitImpl {
            signature,
            classnames,
            output: &self.output,
        };

        let ts = quote! {
            #class_def
            #trait_impl
        };
        ts.to_tokens(tokens);
    }
}
