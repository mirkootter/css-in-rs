extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;

mod data;
mod output;
mod result;

/// Introduces dynamic CSS code which can be injected.
///
/// # Example
/// ```
/// # use css_in_rs_macro::make_styles;
/// #[derive(Clone)]
/// struct MyTheme {
///     pub primary_color: String,
///     pub header_color: String,
/// }
/// impl css_in_rs::Theme for MyTheme {
///     /* ... */
///     # fn fast_cmp(&self, other: &MyTheme) -> bool { false }
/// }
///
/// make_styles! {
///     (theme: MyTheme) -> MyClasses {
///         "*" {
///             padding: "0px",
///         },
///         text {
///             margin: "10px",
///             color: theme.primary_color,
///         },
///         header {
///             margin: "20px",
///             color: theme.header_color,
///         },
///     }
/// }
/// ```
/// This essentially expands to
/// ```
/// // Generated code; expanded form of the macro above
/// struct MyClasses {
///     pub text: String,
///     pub header: String,
/// }
///
/// impl ::css_in_rs::Classes for MyClasses {
///     # type Theme = css_in_rs::EmptyTheme;
///     # fn generate(_a: &css_in_rs::EmptyTheme, _b: &mut String, _c: &mut u64) {}
///     # fn new(_: u64) -> Self { todo!() }
///     /* ... */
/// }
/// ```
/// You can inject this style into the DOM using a `StyleProvider` (see
/// css-in-rs crate). It will hand you a `MyClasses` instance with uniquely
/// generated classnames (usually something like `css-17`).
#[proc_macro]
pub fn make_styles(input: TokenStream) -> TokenStream {
    let style = syn::parse_macro_input!(input as data::Style);

    let result = result::Result::new(style);
    let expanded = result.to_token_stream();

    // Convert the generated code to a TokenStream
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use crate::{data::Style, result};

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
        let result = result::Result::new(style);
        let result = result.to_token_stream().to_string();

        let expected = quote! {
            struct MyClasses {
                pub blue_color: String,
                pub red_color: String,
            }

            impl ::css_in_rs::Classes for MyClasses {
                type Theme = MyTheme;

                fn generate(theme: &Self::Theme, css: &mut String, counter: &mut u64) {
                    use ::core::fmt::Write;
                    let start = *counter;
                    let _ = write!(
                        css,
                        "div.css-{} {{\n  {}: {};\n}}\ndiv.css-{} {{\n  {}: {};\n}}\n",
                        start + 1u64,
                        "color",
                        "red",
                        start + 0u64,
                        "color",
                        "blue"
                    );
                    *counter = start + 2u64;
                }
                fn new(start: u64) -> Self {
                    Self {
                        blue_color: format!("css-{}", start + 0u64),
                        red_color: format!("css-{}", start + 1u64),
                    }
                }
            }
        };

        assert_eq!(result, expected.to_string());
    }
}
