extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{self, DeriveInput, Data, Fields, Ident, Type};
use Data::*;
use Fields::*;

#[proc_macro_derive(Parser)]
pub fn parse_derive(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let name = ast.ident;
    let gen = match ast.data {
        Struct(data) => {
            let sequence = parse_sequence_derive(quote! { #name }, data.fields);

            quote! {
                impl Parser for #name {
                    fn parse(s: &str) -> Option<(&str, #name)> {
                        #sequence
                    }
                }
            }
        },
        Enum(data) => {
            let (methods, parse_vars): (Vec<Ident>, Vec<TokenStream2>) = data.variants
                .into_iter()
                .map(|variant| {
                    let tag = variant.ident;
                    let method = format_ident!("parse_{}", tag);
                    let parse_variant = parse_sequence_derive(quote! { #name::#tag }, variant.fields);
                    (method, parse_variant)
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

fn parse_sequence_derive(name: TokenStream2, fields: Fields) -> TokenStream2 {
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
                .map(|(i, field)| (format_ident!("ident{}", i), field.ty))
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

#[proc_macro_attribute]
pub fn parse_unit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: DeriveInput = syn::parse(item).unwrap();
    let name = item.ident;
    let term = TokenStream2::from(attr);
    let gen = quote! {
        struct #name;
        impl Parser for #name {
            fn parse(s: &str) -> Option<(&str, #name)> {
                let mut chs = s.chars();
                if Some(#term) == chs.next() {
                    Some((chs.as_str(), #name))
                } else {
                    None
                }
            }
        }
    };
    gen.into()
}
