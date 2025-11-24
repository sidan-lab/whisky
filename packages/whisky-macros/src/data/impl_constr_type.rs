use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, Token, Type, TypePath,
};

struct ImplConstrTypeInput {
    type_name: Ident,
    tag: Expr,
    params: Vec<Parameter>,
}

struct Parameter {
    name: Ident,
    ty: Type,
}

impl Parse for ImplConstrTypeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: TypeName, tag, [params]
        let type_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let tag: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Parse the parameter list inside brackets
        let content;
        syn::bracketed!(content in input);

        let mut params = Vec::new();
        while !content.is_empty() {
            // Parse (name: Type)
            let param_content;
            syn::parenthesized!(param_content in content);

            let name: Ident = param_content.parse()?;
            param_content.parse::<Token![:]>()?;
            let ty: Type = param_content.parse()?;

            params.push(Parameter { name, ty });

            // Parse optional comma
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(ImplConstrTypeInput {
            type_name,
            tag,
            params,
        })
    }
}

/// Extract the input type for a type's `new()` method by looking at common patterns
fn infer_new_input_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            let type_name = last_segment.ident.to_string();

            // Handle known types
            match type_name.as_str() {
                "ByteString" | "ScriptHash" | "PolicyId" | "AssetName" | "PubKeyHash" => {
                    quote! { &str }
                }
                "Int" => quote! { i128 },
                "Bool" => quote! { bool },
                "Credential" => quote! { (&str, bool) },
                // For generic types, try to infer from the inner type
                _ => {
                    // Default: use a generic parameter that will be inferred
                    let param_ident = Ident::new(&format!("__{}_Input", type_name), Span::call_site());
                    quote! { #param_ident }
                }
            }
        }
        _ => {
            // For complex types, use a placeholder that will be inferred
            quote! { _ }
        }
    }
}

pub fn impl_constr_type_macro(input: TokenStream) -> TokenStream {
    let ImplConstrTypeInput {
        type_name,
        tag,
        params,
    } = parse_macro_input!(input as ImplConstrTypeInput);

    // Generate parameter names and inferred types
    let param_names: Vec<_> = params.iter().map(|p| &p.name).collect();
    let inferred_input_types: Vec<_> = params.iter().map(|p| infer_new_input_type(&p.ty)).collect();

    // Determine which constructor to use based on the tag
    let (constructor, needs_tag) = match &tag {
        Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Int(_)) => {
            if let syn::Lit::Int(lit_int) = &lit.lit {
                match lit_int.base10_parse::<u32>() {
                    Ok(0) => (quote! { Constr0 }, false),
                    Ok(1) => (quote! { Constr1 }, false),
                    Ok(2) => (quote! { Constr2 }, false),
                    _ => (quote! { Constr }, true),
                }
            } else {
                (quote! { Constr }, true)
            }
        }
        _ => (quote! { Constr }, true),
    };

    // Generate the field initializations: <Type>::new(param_name)
    let field_inits = params.iter().map(|p| {
        let name = &p.name;
        let ty = &p.ty;
        quote! { <#ty>::new(#name) }
    });

    // Generate the impl block
    // Note: We always wrap in Self() to support newtype pattern
    let expanded = if params.len() == 1 {
        // Single parameter - don't wrap in Box or tuple
        let field_init = field_inits.clone().next().unwrap();
        if !needs_tag {
            quote! {
                impl #type_name {
                    pub fn from(#(#param_names: #inferred_input_types),*) -> Self {
                        Self(#constructor::new(#field_init))
                    }
                }
            }
        } else {
            quote! {
                impl #type_name {
                    pub fn from(#(#param_names: #inferred_input_types),*) -> Self {
                        Self(Constr::new(#tag, #field_init))
                    }
                }
            }
        }
    } else if !needs_tag {
        // For tags 0, 1, 2 - use ConstrN::new without tag parameter
        quote! {
            impl #type_name {
                pub fn from(#(#param_names: #inferred_input_types),*) -> Self {
                    Self(#constructor::new(Box::new((
                        #(#field_inits),*
                    ))))
                }
            }
        }
    } else {
        // For other tags - use Constr::new with tag parameter
        quote! {
            impl #type_name {
                pub fn from(#(#param_names: #inferred_input_types),*) -> Self {
                    Self(Constr::new(#tag, Box::new((
                        #(#field_inits),*
                    ))))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
