extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, DeriveInput, Data, Fields, Ident, Type};
use Data::*;
use Fields::*;

#[proc_macro_derive(Parser)]
pub fn parse_derive(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let name = ast.ident;
    let gen = match ast.data {
        Struct(data_struct) => match data_struct.fields {
            Named(fields) => {
                let (idents, types): (Vec<Ident>, Vec<Type>) = fields.named
                    .into_iter()
                    .map(|field| (field.ident.unwrap(), field.ty))
                    .unzip();
                
                quote! {
                    impl Parser for #name {
                        fn parse(s: &str) -> Option<(&str, #name)> {
                            #(let (s, #idents) = #types::parse(s)?;)*
                            Some((s, #name { #(#idents),* }))
                        }
                    }
                }
            },
            Unnamed(fields) => {
                let (idents, types): (Vec<Ident>, Vec<Type>) = fields.unnamed
                    .into_iter().enumerate()
                    .map(|(i, field)| (Ident::new(&format!("ident{}", i), Span::call_site()), field.ty))
                    .unzip();
                
                quote! {
                    impl Parser for #name {
                        fn parse(s: &str) -> Option<(&str, #name)> {
                            #(let (s, #idents) = #types::parse(s)?;)*
                            Some((s, #name ( #(#idents),* )))
                        }
                    }
                }
            },
            Unit => {
                quote! {
                    impl Parser for #name {
                        fn parse(s: &str) -> Option<(&str, #name)> {
                            Some((s, #name))
                        }
                    }
                }
            },
        },
        _ => panic!(),
    };
    gen.into()
}
