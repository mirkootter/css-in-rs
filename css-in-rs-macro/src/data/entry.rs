use syn::parse::{Parse, ParseStream};

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
