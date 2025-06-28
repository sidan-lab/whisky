use proc_macro::TokenStream;

mod data;

#[proc_macro_derive(ConstrEnum)]
pub fn derive_constr_enum(input: TokenStream) -> TokenStream {
    data::enum_constr::derive_plutus_data_to_json(input)
}

#[proc_macro_derive(ConstrWrapper)]
pub fn derive_constr_wrapper(input: TokenStream) -> TokenStream {
    data::constr_wrapper::derive_constr_wrapper(input)
}
