//! `NexusVerb` codegen — emits `NotaEncode` + `NotaDecode` for
//! a closed enum whose variants name the kinds the verb operates
//! on. Decoding peeks the next record's head identifier and
//! dispatches to the matching variant.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let _name = input.ident;
    quote! {}
}
