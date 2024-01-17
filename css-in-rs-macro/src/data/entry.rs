use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub struct Entry {
    pub property: String,
    pub value: syn::Expr,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let property = input.parse::<syn::Ident>()?;
        input.parse::<syn::token::Colon>()?;
        let value = input.parse::<syn::Expr>()?;

        let entry = Entry {
            property: property.to_string(),
            value,
        };

        Ok(entry)
    }
}

impl ToTokens for Entry {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let property = &self.property;
        let value = &self.value;

        let write = quote_spanned! { value.span() =>
            ::core::writeln!(css, "  {}: {};", #property, #value).unwrap();
        };
        write.to_tokens(tokens);
    }
}
