extern crate proc_macro;

use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{self, DeriveInput, Data, Fields, Ident, Type};
use Data::*;
use Fields::*;

#[proc_macro_derive(Parser)]
pub fn parse_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let name = ast.ident;
    let gen = match ast.data {
        Struct(data) => {
            let sequence = parse_sequence_derive(&name, data.fields);

            quote! {
                impl Parser for #name {
                    fn parse(s: &str) -> Option<(&str, #name)> {
                        #sequence
                    }
                }
            }
        },
        Enum(data) => {
            let (methods, parse_vars): (Vec<Ident>, Vec<proc_macro2::TokenStream>) = data.variants
                .into_iter()
                .map(|variant| {
                    let tag = variant.ident;
                    let fields = match variant.fields {
                        Unnamed(fields) => fields,
                        _ => panic!(),
                    };
                    let (idents, types): (Vec<Ident>, Vec<Type>) = fields.unnamed
                        .into_iter().enumerate()
                        .map(|(i, field)| (Ident::new(&format!("ident{}", i), Span::call_site()), field.ty))
                        .unzip();

                    let code = quote! {
                        #(let (s, #idents) = #types::parse(s)?;)*
                        Some((s, #name::#tag( #(#idents),* )))
                    };

                    (format_ident!("parse_{}", tag), code)
                })
                .unzip();

            quote! {
                impl Parser for #name {
                    fn parse(s: &str) -> Option<(&str, #name)> {
                        #(if let Some(res) = #name::#methods(s) { return Some(res); })*
                        None
                    }
                }

                impl #name {
                    #(fn #methods(s: &str) -> Option<(&str, #name)> {
                        #parse_vars
                    })*
                }
            }
        },
        _ => panic!(),
    };
    gen.into()
}

fn parse_sequence_derive(name: &Ident, fields: Fields) -> proc_macro2::TokenStream {
    match fields {
        Named(fields) => {
            let (idents, types): (Vec<Ident>, Vec<Type>) = fields.named
                .into_iter()
                .map(|field| (field.ident.unwrap(), field.ty))
                .unzip();

            quote! {
                #(let (s, #idents) = #types::parse(s)?;)*
                Some((s, #name { #(#idents),* }))
            }
        },
        Unnamed(fields) => {
            let (idents, types): (Vec<Ident>, Vec<Type>) = fields.unnamed
                .into_iter().enumerate()
                .map(|(i, field)| (Ident::new(&format!("ident{}", i), Span::call_site()), field.ty))
                .unzip();

            quote! {
                #(let (s, #idents) = #types::parse(s)?;)*
                Some((s, #name ( #(#idents),* )))
            }
        },
        Unit => {
            quote! { Some((s, #name)) }
        },
    }
}
