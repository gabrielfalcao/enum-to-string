use syn::Error;
use proc_macro2::{Span, TokenStream};

pub(crate) fn compile_error_from_span<T>(error: String, span: Span) -> Result<T, TokenStream> {
    Err(Error::new(span, error).to_compile_error())
}

pub(crate) fn compile_error<T>(error: String) -> Result<T, TokenStream> {
    compile_error_from_span(error, Span::call_site())
}

pub(crate) fn compile_error_non_enum<T>(
    macro_name: &str,
    actual_type: &str,
    source_text: String,
) -> Result<T, TokenStream> {
    compile_error(format!(
        "#[derive({})] does not work for {} types as appears to be the case: {:#?}",
        macro_name, actual_type, source_text
    ))
}

#[allow(unused)]
pub(crate) fn debug_tokens(gossip: TokenStream) {
    ec(
        format!("{:#?}", &gossip)
            .lines()
            .map(|n| format!("\x1b[1;38;5;220m{}\x1b[0m", n))
            .collect::<Vec<String>>()
            .join("\n"),
        220,
    );
}

#[allow(unused)]
pub(crate) fn ec(s: impl std::fmt::Display, c: u8) {
    eprintln!("\x1b[1;38;5;{}m{}\x1b[0m", c, s);
}
