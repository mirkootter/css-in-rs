use std::collections::{btree_map::Entry, BTreeMap};

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

use crate::output::{Output, ToOutput};

#[derive(Debug)]
pub enum Part {
    Raw(String),
    ClassName(String),
}

#[derive(Debug)]
pub struct Header {
    pub parts: Vec<Part>,
    pub span: Span,
}

impl Header {
    pub fn collect_classnames(&self, result: &mut BTreeMap<String, Span>) {
        for part in &self.parts {
            match part {
                Part::Raw(_) => {}
                Part::ClassName(classname) => {
                    let classname = classname.to_string();
                    match result.entry(classname) {
                        Entry::Vacant(vac) => {
                            vac.insert(self.span.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

mod parse {
    use proc_macro2::Span;

    use super::{Header, Part};

    type ParseResult<'a, T> = nom::IResult<&'a str, T>;

    pub fn parse(src: &str, span: Span) -> Option<Header> {
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

        let header = Header { parts, span };
        Some(header)
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

impl Parse for Header {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let header = if let Ok(ident) = input.parse::<syn::Ident>() {
            let part = Part::ClassName(ident.to_string());
            let span = ident.span();
            Header {
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

        Ok(header)
    }
}

impl ToOutput for Part {
    fn append(&self, result: &mut Output) {
        match self {
            Part::Raw(s) => result.push_str(s),
            Part::ClassName(s) => {
                result.push_str(".");
                result.push_classname(s);
            }
        }
    }
}

impl ToOutput for Header {
    fn append(&self, result: &mut Output) {
        for part in &self.parts {
            part.append(result);
        }
    }
}
