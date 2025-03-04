#![allow(unused)]
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Error, Ident, Variant};

use crate::{
    compile_error, compile_error_non_enum, ident_to_string, make_match_to_str_variants,
    make_match_to_variants, EnumToStringAttrs,
};
pub(crate) struct DeriveGen {
    attrs: EnumToStringAttrs,
    ast: DeriveInput,
}

impl DeriveGen {
    pub fn enum_ident(&mut self) -> Ident {
        self.ast.ident.clone()
    }

    pub fn ast(&mut self) -> DeriveInput {
        self.ast.clone()
    }

    pub fn ast_data(&mut self) -> Data {
        self.ast.data.clone()
    }

    pub fn parse_input(input: proc_macro::TokenStream) -> Result<DeriveGen, TokenStream> {
        let ast: DeriveInput = syn::parse(input).map_err(|e| e.to_compile_error())?;
        validate_enum(&ast)?;
        let attrs = EnumToStringAttrs::parse_from_macro_attrs(&ast.attrs)?;
        Ok(DeriveGen { attrs, ast })
    }

    pub fn can_impl_display_enum(&mut self) -> bool {
        self.attrs.display()
    }

    pub fn can_impl_debug_enum(&mut self) -> bool {
        self.attrs.debug()
    }

    pub fn can_impl_variants_enum(&mut self) -> bool {
        self.attrs.variants()
    }

    pub fn impl_display_enum(&mut self) -> TokenStream {
        let name = &self.enum_ident();
        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        self.as_str()
                    )
                }
            }
        }
    }

    pub fn impl_debug_enum(&mut self) -> TokenStream {
        let name = &self.enum_ident();
        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        self.as_dbg()
                    )
                }
            }
        }
    }

    pub fn extract_enum_variants(&mut self) -> Vec<Variant> {
        if let Data::Enum(l) = &self.ast_data() {
            return l.variants.iter().map(|v| v.clone()).collect::<Vec<_>>();
        } else {
            Vec::new()
        }
    }

    pub fn impl_enum_to_string_variants(&mut self) -> (TokenStream, TokenStream, TokenStream) {
        let name = self.enum_ident();
        let str_variants = make_match_to_str_variants(&self.ast(), &|_index_: usize, variant: &Variant| {
            ident_to_string(&variant.ident).to_lowercase()
        });
        let dbg_variants = make_match_to_str_variants(&self.ast(), &|_index_: usize, variant: &Variant| {
            format!(
                "{}::{}",
                ident_to_string(&name),
                ident_to_string(&variant.ident)
            )
        });

        let impl_variants = if self.can_impl_variants_enum() {
            let vl = make_match_to_variants(&self.ast(), &|_index_: usize, variant: &Variant| {
                quote! { #name::#variant }
            });
            quote! {
                impl #name {
                    pub fn variants<'a>() -> &'a [#name] {
                        &[
                            #vl
                        ]
                    }
                }
                impl std::cmp::PartialEq for #name {
                    fn eq(&self, other: &Self) -> bool {
                        self.to_string().eq(&other.to_string())
                    }
                }
                impl std::cmp::Eq for #name {}
                impl std::cmp::PartialOrd for #name {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        self.to_string().partial_cmp(&other.to_string())
                    }
                }
                impl std::cmp::Ord for #name {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        self.to_string().cmp(&other.to_string())
                    }
                }

                impl std::hash::Hash for #name {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        self.to_string().hash(state);
                    }
                }
            }
        } else {
            TokenStream::new()
        };
        (impl_variants, str_variants, dbg_variants)
    }
}

pub fn validate_enum(ast: &DeriveInput) -> Result<(), TokenStream> {
    match &ast.data {
        Data::Enum(_) => Ok(()),
        Data::Union(u) => compile_error_non_enum(
            "EnumToString",
            "Union",
            u.union_token.span.source_text().unwrap(),
        ),
        Data::Struct(s) => compile_error_non_enum(
            "EnumToString",
            "Struct",
            s.struct_token.span.source_text().unwrap(),
        ),
    }
}
