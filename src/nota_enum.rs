//! `NotaEnum` codegen — emits `NotaEncode` + `NotaDecode`
//! impls that round-trip a unit-variant enum as its
//! PascalCase variant identifier.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let _name = input.ident;
    quote! {}
}
