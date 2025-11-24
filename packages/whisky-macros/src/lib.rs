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

#[proc_macro]
pub fn impl_constr_type(input: TokenStream) -> TokenStream {
    data::impl_constr_type::impl_constr_type_macro(input)
}

#[proc_macro_derive(ImplConstr, attributes(constr))]
pub fn derive_impl_constr(input: TokenStream) -> TokenStream {
    data::impl_constr_derive::derive_impl_constr(input)
}
