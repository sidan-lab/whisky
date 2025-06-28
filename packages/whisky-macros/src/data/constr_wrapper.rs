use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_constr_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ::whisky::data::PlutusDataJson for #name {
            fn to_json(&self) -> Value {
                self.0.to_json()
            }

            fn to_json_string(&self) -> String {
                self.to_json().to_string()
            }

            fn to_constr_field(&self) -> Vec<Value> {
                vec![self.0.to_json()]
            }
        }

    };

    expanded.into()
}
