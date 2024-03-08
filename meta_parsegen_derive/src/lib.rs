extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

#[proc_macro_derive(Parser)]
pub fn parse_derive(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl Parser for #name {
            fn parse(s: &str) -> #name {
                #name {}
            }
        }
    };
    gen.into()
}
