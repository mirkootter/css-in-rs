extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

mod data;
mod output;

#[proc_macro]
pub fn make_styles(input: TokenStream) -> TokenStream {
    let style = syn::parse_macro_input!(input as data::Style);

    // Generate the code using quote!()
    let expanded = {
        let def = style.class_definition();
        let trait_impl = style.trait_impl();
        quote! {
            #def
            #trait_impl
        }
    };

    // Convert the generated code to a TokenStream
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::data::Style;

    #[test]
    fn simple() {
        let input = quote! {
            (theme: MyTheme) -> MyClasses {
                "div.red_color" {
                    color: "red",
                },
                "div.blue_color" {
                    color: "blue",
                },
            }
        };

        let style = syn::parse2::<Style>(input).unwrap();

        let class_def = style.class_definition();
        let class_def = quote!(#class_def).to_string();

        let expected = quote! {
            struct MyClasses {
                pub blue_color: String,
                pub red_color: String,
            }
        };

        assert_eq!(class_def, expected.to_string());

        let trait_impl = style.trait_impl();
        let trait_impl = quote!(#trait_impl).to_string();

        let expected = quote! {
            impl ::css_in_rs::Classes for MyClasses {
                type Theme = MyTheme;

                fn generate(theme: &Self::Theme, css: &mut String, counter: &mut u64) {
                    use ::core::fmt::Write;
                    let start = *counter;
                    ::core::writeln!(css, "div.css-{} {{", start + 1u64).unwrap();
                    ::core::writeln!(css, "  {}: {};", "color", "red").unwrap();
                    ::core::writeln!(css, "}}").unwrap();
                    ::core::writeln!(css, "div.css-{} {{", start + 0u64).unwrap();
                    ::core::writeln!(css, "  {}: {};", "color", "blue").unwrap();
                    ::core::writeln!(css, "}}").unwrap();
                    *counter = start + 2u64;
                }

                fn new(start: u64) -> Self {
                    Self {
                        blue_color: ::std::format!("css-{}", start + 0u64),
                        red_color: ::std::format!("css-{}", start + 1u64),
                    }
                }
            }
        };

        assert_eq!(trait_impl, expected.to_string());
    }
}
