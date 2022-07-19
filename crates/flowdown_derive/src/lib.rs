extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, parse::Parser, Data, Field};
use quote::quote;

// from https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307/2
#[proc_macro_attribute]
pub fn base_line(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(Field::parse_named.parse2(quote!{
                        pub id: String,
                        pub line_type: String,
                        pub next_id: Option<String>,
                    }).unwrap());
                },
                _ => {}
            }

            return quote! {
                #ast
            }.into();
        },
        _ => panic!("only use on structs")
    }
}

