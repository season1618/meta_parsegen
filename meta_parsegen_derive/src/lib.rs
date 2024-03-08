extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Data, Ident, Type};
use Data::*;

#[proc_macro_derive(Parser)]
pub fn parse_derive(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let name = ast.ident;
    let gen = match ast.data {
        Struct(data_struct) => {
            let (idents, types): (Vec<Ident>, Vec<Type>) = data_struct.fields
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
    };
    gen.into()
}
