use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub enum Part {
    Raw(String),
    ClassName(String),
}

#[derive(Debug)]
pub struct Selector {
    pub parts: Vec<Part>,
    pub span: Span,
}

impl Parse for Selector {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let selector = if let Ok(ident) = input.parse::<syn::Ident>() {
            let part = Part::ClassName(ident.to_string());
            let span = ident.span();
            Selector {
                parts: vec![part],
                span,
            }
        } else {
            let source = input.parse::<syn::LitStr>()?;
            // TODO
            return Err(syn::Error::new(source.span(), "Parsing not supported yet."));
        };

        Ok(selector)
    }
}
