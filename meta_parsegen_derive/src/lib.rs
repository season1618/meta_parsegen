extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
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
