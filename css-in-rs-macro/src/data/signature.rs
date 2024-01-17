use syn::parse::{Parse, ParseStream};

pub struct Signature {
    pub theme_varname: syn::Ident,
    pub theme_type: syn::Ident,
    pub classname: syn::Ident,
}

impl Parse for Signature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (theme_varname, theme_type) = {
            let param;
            syn::parenthesized!(param in input);

            let theme_varname = param.parse::<syn::Ident>()?;
            param.parse::<syn::token::Colon>()?;
            let theme_type = param.parse::<syn::Ident>()?;

            (theme_varname, theme_type)
        };

        input.parse::<syn::token::RArrow>()?;
        let classname = input.parse::<syn::Ident>()?;

        let signature = Signature {
            theme_varname,
            theme_type,
            classname,
        };
        Ok(signature)
    }
}
