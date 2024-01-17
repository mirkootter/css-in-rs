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

mod parse {
    use proc_macro2::Span;

    use super::{Part, Selector};

    type ParseResult<'a, T> = nom::IResult<&'a str, T>;

    pub fn parse(src: &str, span: Span) -> Option<Selector> {
        let mut src = src.trim();

        let mut parts = Vec::new();
        loop {
            let (remaining, part) = parse_part(src).ok()?;
            parts.push(part);

            src = remaining;
            if src.is_empty() {
                break;
            }
        }

        let sel = Selector { parts, span };
        Some(sel)
    }

    fn parse_classname(src: &str) -> ParseResult<&str> {
        let (src, _) = nom::bytes::complete::tag(".")(src)?;
        let (src, classname) =
            nom::bytes::complete::take_while1(|ch: char| ch.is_ascii_alphanumeric() || ch == '_')(
                src,
            )?;

        if src.starts_with('-') {
            // '-' is not allowed in identifiers
            let err = nom::error::make_error(src, nom::error::ErrorKind::Fail);
            return Err(nom::Err::Failure(err));
        }

        Ok((src, classname))
    }

    fn parse_part(src: &str) -> ParseResult<Part> {
        if src.starts_with('.') {
            let (src, classname) = parse_classname(src)?;
            let part = Part::ClassName(classname.to_string());
            return Ok((src, part));
        }

        let (src, chunk) = nom::bytes::complete::take_while1(|ch: char| ch != '.')(src)?;
        let part = Part::Raw(chunk.to_string());
        Ok((src, part))
    }
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

            if let Some(sel) = parse::parse(&source.value(), source.span()) {
                return Ok(sel);
            } else {
                return Err(syn::Error::new(
                    source.span(),
                    "Parsing failed. Not a valid selector",
                ));
            }
        };

        Ok(selector)
    }
}
