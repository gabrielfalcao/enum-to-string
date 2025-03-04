use std::collections::BTreeMap;

use quote::ToTokens;
use proc_macro2::{TokenStream, TokenTree};
use syn::{Attribute, Meta};

use crate::compile_error_from_span;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnumToStringAttrs {
    impl_debug: bool,
    impl_display: bool,
    impl_variants: bool,
}

impl EnumToStringAttrs {
    pub fn new() -> EnumToStringAttrs {
        EnumToStringAttrs {
            impl_display: true,
            impl_debug: true,
            impl_variants: true,
        }
    }
    pub fn from_flags(flags: BTreeMap<String, bool>) -> EnumToStringAttrs {
        let mut ets = EnumToStringAttrs::new();
        if let Some(display) = flags.get("display") {
            ets.set_impl_display(*display);
        }
        if let Some(debug) = flags.get("debug") {
            ets.set_impl_debug(*debug);
        }
        if let Some(variants) = flags.get("variants") {
            ets.set_impl_variants(*variants);
        }
        ets
    }
    pub fn flags_from_tokenstream(
        stream: TokenStream,
    ) -> Result<BTreeMap<String, bool>, TokenStream> {
        EnumToStringAttrs::flags_from_iter_tokentree(stream.into_iter())
    }
    pub fn flags_from_iter_tokentree(
        stream: impl Iterator<Item = TokenTree>,
    ) -> Result<BTreeMap<String, bool>, TokenStream> {
        let mut flags = BTreeMap::<String, bool>::new();
        let mut pattrs = Vec::<String>::new();
        let mut xpecteq = false;
        let mut xpectcomma = false;
        let mut xpectflag = true;
        let mut xpectbool = false;

        for tok in stream {
            match &tok {
                TokenTree::Ident(name) => {
                    let name = name.span().source_text().unwrap();
                    if xpecteq {
                        return compile_error_from_span(format!("expecting: ="), tok.span());
                    }

                    match name.as_str() {
                        "debug" | "display" | "variants" => {
                            xpectbool = true;
                            xpectcomma = false;
                            xpectflag = false;
                            xpecteq = true;
                            pattrs.push(name.clone());
                        },
                        "true" | "false" => {
                            if !xpectbool || xpectflag || pattrs.is_empty() {
                                return compile_error_from_span(
                                    format!("expecting bool: xpectbool: {xpectbool}"),
                                    tok.span(),
                                );
                            }
                            let flag = pattrs.pop().unwrap();
                            flags.insert(flag.clone(), name.as_str() == "true");

                            xpectbool = false;
                            xpectcomma = true;
                            xpecteq = false;
                            xpectflag = true;
                        },
                        _ =>
                            return compile_error_from_span(format!("expecting ident"), tok.span()),
                    };
                },
                TokenTree::Punct(p) => match p.as_char() {
                    ',' => {
                        if !xpectcomma {
                            return compile_error_from_span(
                                format!("expecting `,` found `{}`", p.as_char()),
                                tok.span(),
                            );
                        }
                        xpecteq = false;
                        xpectcomma = false;
                        xpectbool = false;
                        xpectflag = true;
                    },
                    '=' => {
                        if !xpecteq {
                            return compile_error_from_span(
                                format!("expecting `=` found `{}`", p.as_char()),
                                tok.span(),
                            );
                        }

                        xpecteq = false;
                        xpectcomma = false;
                        xpectbool = true;
                        xpectflag = false;
                    },
                    _ =>
                        return compile_error_from_span(
                            format!("unexpected punctuation: {}", p.as_char()),
                            tok.span(),
                        ),
                },
                TokenTree::Group(_) => {
                    let span = tok.span();
                    return compile_error_from_span(
                        format!(
                            "unexpected group{}",
                            match span.source_text() {
                                Some(text) => format!(": {}", text),
                                None => String::new(),
                            }
                        ),
                        span,
                    );
                },
                TokenTree::Literal(_) => {
                    let span = tok.span();
                    return compile_error_from_span(
                        format!(
                            "unexpected literal{}",
                            match span.source_text() {
                                Some(text) => format!(": {}", text),
                                None => String::new(),
                            }
                        ),
                        span,
                    );
                },
            }
        }
        Ok(flags)
    }

    pub fn parse_from_macro_attrs(
        attrs: &Vec<Attribute>,
    ) -> Result<EnumToStringAttrs, TokenStream> {
        let mut flags = BTreeMap::<String, bool>::new();
        for attr in attrs {
            match &attr.meta {
                Meta::List(ml) => {
                    for (k, v) in EnumToStringAttrs::flags_from_iter_tokentree(
                        &mut ml.clone().tokens.into_iter(),
                    )? {
                        flags.insert(k, v);
                    }
                },
                Meta::Path(n) => {
                    let mut stream = TokenStream::new();
                    n.to_tokens(&mut stream);
                },
                Meta::NameValue(n) => {
                    let mut stream = TokenStream::new();
                    n.to_tokens(&mut stream);
                },
            }
        }
        Ok(EnumToStringAttrs::from_flags(flags))
    }

    pub fn set_impl_display(&mut self, display: bool) {
        self.impl_display = display;
    }
    pub fn display(&self) -> bool {
        self.impl_display
    }
    pub fn set_impl_debug(&mut self, debug: bool) {
        self.impl_debug = debug;
    }
    pub fn debug(&self) -> bool {
        self.impl_debug
    }
    pub fn set_impl_variants(&mut self, variants: bool) {
        self.impl_variants = variants;
    }
    pub fn variants(&self) -> bool {
        self.impl_variants
    }
}
