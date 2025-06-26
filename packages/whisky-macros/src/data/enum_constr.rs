use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_plutus_data_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let implementation = match &input.data {
        Data::Enum(data_enum) => {
            let match_arms = data_enum.variants.iter().enumerate().map(|(index, variant)| {
                let variant_name = &variant.ident;
                let full_variant_path = quote! { #name::#variant_name };
                
                match &variant.fields {
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        // Single field tuple variant like UserSpotAccount(Account)
                        quote! {
                            #full_variant_path(field) => ::whisky_common::data::Constr::new(#index as u64, field.clone()).to_json()
                        }
                    }
                    Fields::Named(_) => {
                        // Named fields - you can extend this if needed
                        panic!("Named fields not supported yet");
                    }
                    Fields::Unit => {
                        // Unit variant like SomeVariant
                        quote! {
                            #full_variant_path => ::whisky_common::data::Constr::new(#index as u64, ()).to_json()
                        }
                    }
                    _ => {
                        panic!("Unsupported field type");
                    }
                }
            });

            quote! {
                impl ::whisky_common::data::PlutusDataToJson for #name {
                    fn to_json(&self) -> ::serde_json::Value {
                        match self {
                            #(#match_arms,)*
                        }
                    }
                    
                    fn to_json_string(&self) -> String {
                        self.to_json().to_string()
                    }
                }
            }
        }
        _ => {
            panic!("PlutusDataToJson can only be derived for enums");
        }
    };

    implementation.into()
}