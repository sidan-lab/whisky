use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, TypeParamBound,
};

pub fn derive_plutus_data_to_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract generics and add PlutusDataJson bound to all type parameters
    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            let bound: TypeParamBound = parse_quote!(whisky::data::PlutusDataJson);
            type_param.bounds.push(bound);
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = match &input.data {
        Data::Enum(data_enum) => {
            let match_arms = data_enum.variants.iter().enumerate().map(|(index, variant)| {
                let variant_name = &variant.ident;
                let full_variant_path = quote! { #name::#variant_name };
                match &variant.fields {
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        // Single field tuple variant like UserSpotAccount(Account)
                        quote! {
                            #full_variant_path(field) => whisky::data::Constr::new(#index as u64, field.clone()).to_json()
                        }
                    }
                    Fields::Unnamed(fields) => {
                        // Multiple fields tuple variant like MintCancelOrderIntent(UserAccount, ByteString)
                        let field_count = fields.unnamed.len();
                        let field_names: Vec<_> = (0..field_count).map(|i| syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site())).collect();
                        let pattern = quote! { #(#field_names),* };
                        let tuple = quote! { (#(#field_names.clone()),*) };
                        quote! {#full_variant_path(#pattern) => whisky::data::Constr::new(#index as u64, Box::new(#tuple)).to_json()}
                    }
                    Fields::Named(_) => {
                        // Named fields - you can extend this if needed
                        panic!("Named fields not supported yet");
                    }
                    Fields::Unit => {
                        // Unit variant like SomeVariant
                        quote! {
                            #full_variant_path => whisky::data::Constr::new(#index as u64, ()).to_json()
                        }
                    }
                }
            });

            quote! {
                impl #impl_generics whisky::data::PlutusDataJson for #name #ty_generics #where_clause {
                    fn to_json(&self) -> ::serde_json::Value {
                        match self {
                            #(#match_arms,)*
                        }
                    }

                    fn to_json_string(&self) -> String {
                        self.to_json().to_string()
                    }

                    fn to_constr_field(&self) -> Vec<::serde_json::Value> {
                        vec![self.to_json()]
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
