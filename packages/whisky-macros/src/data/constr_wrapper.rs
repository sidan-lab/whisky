use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_constr_wrapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract the inner type from the newtype struct
    let inner_ty = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed.first().unwrap().ty
            }
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "ConstrWrapper can only be derived for newtype structs with a single unnamed field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "ConstrWrapper can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl ::whisky::data::PlutusDataJson for #name {
            fn to_json(&self) -> ::serde_json::Value {
                self.0.to_json()
            }

            fn to_json_string(&self) -> String {
                self.to_json().to_string()
            }

            fn to_constr_field(&self) -> Vec<::serde_json::Value> {
                vec![self.0.to_json()]
            }

            fn from_json(value: &::serde_json::Value) -> Result<Self, ::whisky::WError> {
                let inner = <#inner_ty>::from_json(value)
                    .map_err(::whisky::WError::add_err_trace(concat!(stringify!(#name), "::from_json")))?;
                Ok(#name(inner))
            }
        }

    };

    expanded.into()
}
