//! `NexusPattern` codegen — emits `NotaEncode` + `NotaDecode`
//! that round-trip a `*Query` struct as nexus pattern-record
//! form `(| RecordName field0 field1 … |)`. The data record
//! name comes from the required `#[nota(queries = "Name")]`
//! attribute.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let _name = input.ident;
    quote! {}
}
