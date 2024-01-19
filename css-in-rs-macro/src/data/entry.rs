use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::output::{Output, ToOutput};

pub struct Entry {
    pub property: String,
    pub value: syn::Expr,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let property = {
            if let Some(property) = input.parse::<syn::LitStr>().ok() {
                property.value()
            } else {
                let property = input.parse::<syn::Ident>()?;
                property.to_string().replace('_', "-")
            }
        };

        input.parse::<syn::token::Colon>()?;
        let value = input.parse::<syn::Expr>()?;

        let entry = Entry { property, value };

        Ok(entry)
    }
}

impl ToOutput for Entry {
    fn append(&self, result: &mut Output) {
        let property = &self.property;
        let value = &self.value;

        result.format_str.push_str("  {}: {};\n");
        quote!(, #property, #value).to_tokens(&mut result.params);
    }
}
