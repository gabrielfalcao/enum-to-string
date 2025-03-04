#![feature(proc_macro_hygiene)]
#![feature(proc_macro_span)]

extern crate proc_macro;
mod attrs;
mod ast;
mod dgen;
mod error;

use attrs::*;
use ast::*;
use dgen::*;
use error::*;
use quote::quote;
use proc_macro2::TokenStream;


#[proc_macro_derive(EnumToString)]
pub fn enum_to_string_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (mut dgen, basic) = match impl_enum_to_string_macro_methods(input) {
        Ok(t) => t,
        Err(e) => return e.into()
    };

    let debug_impl = if dgen.can_impl_debug_enum() {
        dgen.impl_debug_enum()
    } else {
        TokenStream::new()
    };
    let display_impl = if dgen.can_impl_display_enum() {
        dgen.impl_display_enum()
    } else {
        TokenStream::new()
    };


    quote! {
        #basic
        #display_impl
        #debug_impl
    }.into()
}

#[proc_macro_attribute]
pub fn enum_to_string(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if let Err(e) = EnumToStringAttrs::flags_from_tokenstream(attrs.into()) {
        return e.into()
    }
    item
}


fn impl_enum_to_string_macro_methods(input: proc_macro::TokenStream) -> Result<(DeriveGen, TokenStream), TokenStream> {
    let mut dgen = match DeriveGen::parse_input(input) {
        Ok(dgen) => dgen,
        Err(e) => return Err(e.into()),
    };

    let name = dgen.enum_ident();
    let (impl_variants, str_variant_matches, dbg_variant_matches) = dgen.impl_enum_to_string_variants();

    Ok((dgen, quote! {
        #impl_variants
        impl EnumToString for #name {
            fn as_str(&self) -> &'static str {
                match self {
                    #str_variant_matches
                }
            }
            fn as_dbg(&self) -> &'static str {
                match self {
                    #dbg_variant_matches
                }
            }
        }
    }))
}
