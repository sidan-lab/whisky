use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_json_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ::whisky::data::ToJsonArray for #name {
            fn to_json_array(&self) -> Vec<Value> {
                vec![self.0.to_json()]
            }
        }
    };

    expanded.into()
}
