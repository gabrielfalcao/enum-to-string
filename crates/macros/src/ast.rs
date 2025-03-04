use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Variant};

pub(crate) fn ident_to_string(ident: &Ident) -> String {
    ident.span().unwrap().source_text().unwrap()
}

pub(crate) fn make_match_to_variants<F>(ast: &DeriveInput, to_token_stream: &F) -> TokenStream
where
    F: Fn(usize, &Variant) -> TokenStream,
{
    let variants = extract_enum_variants(&ast)
        .iter()
        .enumerate()
        .map(|(index, var)| {
            to_token_stream(index, &var)
        })
        .collect::<Vec<TokenStream>>();
    quote! {
        #( #variants ),*
    }
}


pub(crate) fn make_match_to_str_variants<F>(ast: &DeriveInput, to_str: &F) -> TokenStream
where
    F: Fn(usize, &Variant) -> String,
{
    let name = ast.ident.clone();
    let variants = extract_enum_variants(&ast)
        .iter()
        .enumerate()
        .map(|(index, var)| {
            let variant_str = to_str(index, &var);
            quote! { #name::#var => #variant_str }
        })
        .collect::<Vec<TokenStream>>();
    quote! {
        #( #variants ),*
    }
}

pub(crate) fn extract_enum_variants(ast: &DeriveInput) -> Vec<Variant> {
    if let Data::Enum(l) = &ast.data {
        return l.variants.iter().map(|v| v.clone()).collect::<Vec<_>>();
    } else {
        Vec::new()
    }
}
