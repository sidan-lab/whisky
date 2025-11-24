use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, TypePath,
};

/// Extract the input type for a type's `new()` method by looking at common patterns
fn infer_new_input_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            let type_name = last_segment.ident.to_string();

            // Handle Box<T> specially - look inside
            if type_name == "Box" {
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        // Recurse to get the inner type's input
                        return infer_new_input_type(inner_ty);
                    }
                }
            }

            // Handle List<T> - takes a Vec<T>
            if type_name == "List" {
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return quote! { Vec<#inner_ty> };
                    }
                }
                return quote! { _ };
            }

            // Handle known types
            match type_name.as_str() {
                "ByteString" | "ScriptHash" | "PolicyId" | "AssetName" | "PubKeyHash" => {
                    quote! { &str }
                }
                "Int" => quote! { i128 },
                "Bool" => quote! { bool },
                "Credential" => quote! { (&str, bool) },
                _ => quote! { #ty }, // For unknown types (wrapper structs), use the type itself
            }
        }
        _ => {
            // For non-path types, return the type as-is
            quote! { #ty }
        }
    }
}

/// Check if a type is a known primitive that has a `new()` method
fn is_known_type_with_new(type_name: &str) -> bool {
    matches!(
        type_name,
        "ByteString"
            | "ScriptHash"
            | "PolicyId"
            | "AssetName"
            | "PubKeyHash"
            | "Int"
            | "Bool"
            | "Credential"
            | "List"
    )
}

/// Generate the field initialization code for a given type
fn generate_field_init(ty: &Type, param_name: &syn::Ident) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            let type_name = last_segment.ident.to_string();

            // Handle Box<T> specially - Box the inner value
            if type_name == "Box" {
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner_init = generate_field_init(inner_ty, param_name);
                        return quote! { Box::new(#inner_init) };
                    }
                }
            }

            // Handle List<T> specially - pass as slice
            if type_name == "List" {
                return quote! { <#ty>::new(&#param_name) };
            }

            // For known primitive types, call new()
            if is_known_type_with_new(&type_name) {
                return quote! { <#ty>::new(#param_name) };
            }

            // For unknown types (likely wrapper structs), just pass the value directly
            // This allows the caller to pass already-constructed wrapper instances
            quote! { #param_name }
        }
        _ => quote! { #param_name },
    }
}

/// Extract tuple fields from Box<(A, B, C)> or (A, B, C)
fn extract_tuple_fields(ty: &Type) -> Option<Vec<Type>> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last()?;

            // Check if this is Box<...>
            if last_segment.ident == "Box" {
                if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        // Recurse into Box to get the inner type
                        return extract_tuple_fields(inner_ty);
                    }
                }
            }
            None
        }
        Type::Tuple(tuple) => {
            // Extract all fields from the tuple
            Some(tuple.elems.iter().cloned().collect())
        }
        _ => None,
    }
}

/// Determine constructor type and tag from the field type
fn analyze_constructor(ty: &Type) -> Option<(String, Option<u32>, Vec<Type>)> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last()?;
            let constr_name = last_segment.ident.to_string();

            // Determine tag from constructor name
            let (tag, _needs_explicit_tag) = match constr_name.as_str() {
                "Constr0" => (Some(0), false),
                "Constr1" => (Some(1), false),
                "Constr2" => (Some(2), false),
                "Constr3" => (Some(3), false),
                "Constr4" => (Some(4), false),
                "Constr5" => (Some(5), false),
                "Constr6" => (Some(6), false),
                "Constr7" => (Some(7), false),
                "Constr8" => (Some(8), false),
                "Constr9" => (Some(9), false),
                "Constr10" => (Some(10), false),
                "Constr" => (None, true), // Needs explicit tag from attribute
                _ => return None,
            };

            // Extract inner type from Constr<T>
            if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    // Try to extract tuple fields
                    let fields =
                        extract_tuple_fields(inner_ty).unwrap_or_else(|| vec![inner_ty.clone()]);
                    return Some((constr_name, tag, fields));
                }
            }

            None
        }
        _ => None,
    }
}

pub fn derive_impl_constr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Check for #[constr(tag = N)] attribute for custom Constr types
    let custom_tag = input.attrs.iter().find_map(|attr| {
        if attr.path.is_ident("constr") {
            attr.parse_args::<syn::LitInt>()
                .ok()
                .and_then(|lit| lit.base10_parse::<u32>().ok())
        } else {
            None
        }
    });

    // Extract the newtype field
    let field_ty = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed.first().unwrap().ty
            }
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "ImplConstr can only be derived for newtype structs with a single unnamed field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "ImplConstr can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Analyze the constructor type
    let (constr_name, tag_opt, fields) = match analyze_constructor(field_ty) {
        Some(result) => result,
        None => {
            return syn::Error::new_spanned(
                field_ty,
                "Expected a Constr0, Constr1, Constr2, or Constr type",
            )
            .to_compile_error()
            .into();
        }
    };

    // Determine the final tag
    let tag = if let Some(t) = tag_opt {
        t
    } else if let Some(t) = custom_tag {
        t
    } else {
        return syn::Error::new_spanned(
            name,
            "Constr type requires #[constr(tag)] attribute to specify the constructor tag",
        )
        .to_compile_error()
        .into();
    };

    // Generate parameter names and inferred types
    let param_count = fields.len();
    let param_names: Vec<_> = (0..param_count)
        .map(|i| syn::Ident::new(&format!("arg{}", i), Span::call_site()))
        .collect();
    let inferred_types: Vec<_> = fields.iter().map(infer_new_input_type).collect();

    // Generate field initializations
    let field_inits: Vec<_> = fields
        .iter()
        .zip(&param_names)
        .map(|(ty, name)| generate_field_init(ty, name))
        .collect();

    // Generate the implementation based on constructor type and field count
    let constr_ident = syn::Ident::new(&constr_name, Span::call_site());

    let constructor_call = if param_count == 1 {
        // Single field - no Box or tuple wrapper
        let field_init = &field_inits[0];
        if constr_name == "Constr" {
            quote! { #constr_ident::new(#tag, #field_init) }
        } else {
            quote! { #constr_ident::new(#field_init) }
        }
    } else {
        // Multiple fields - wrap in Box<tuple>
        if constr_name == "Constr" {
            quote! {
                #constr_ident::new(#tag, Box::new((
                    #(#field_inits),*
                )))
            }
        } else {
            quote! {
                #constr_ident::new(Box::new((
                    #(#field_inits),*
                )))
            }
        }
    };

    let expanded = quote! {

        impl #name {
            pub fn from(#(#param_names: #inferred_types),*) -> Self {
                Self(#constructor_call)
            }
        }

        // Also implement PlutusDataJson trait (ConstrWrapper functionality)
        // The trait must be in scope for this to work, which is expected since
        // users need it imported to use these types anyway
        #[automatically_derived]
        impl PlutusDataJson for #name {
            fn to_json(&self) -> ::serde_json::Value {
                self.0.to_json()
            }

            fn to_json_string(&self) -> ::std::string::String {
                self.to_json().to_string()
            }

            fn to_constr_field(&self) -> ::std::vec::Vec<::serde_json::Value> {
                ::std::vec![self.0.to_json()]
            }
        }

        // Also implement PlutusDataJson for Box<Type> to support nested boxing
        #[automatically_derived]
        impl PlutusDataJson for ::std::boxed::Box<#name> {
            fn to_json(&self) -> ::serde_json::Value {
                self.as_ref().to_json()
            }

            fn to_json_string(&self) -> ::std::string::String {
                self.to_json().to_string()
            }

            fn to_constr_field(&self) -> ::std::vec::Vec<::serde_json::Value> {
                ::std::vec![self.to_json()]
            }
        }
    };

    TokenStream::from(expanded)
}
