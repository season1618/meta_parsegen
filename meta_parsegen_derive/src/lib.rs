extern crate proc_macro;

use proc_macro::TokenStream;
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
                        fn parse(s: &str) -> (&str, #name) {
                            #(let (s, #idents) = #types::parse(s);)*
                            (s, #name { #(#idents),* })
                        }
                    }
                }
            },
            _ => panic!(),
        },
        _ => panic!(),
    };
    gen.into()
}
