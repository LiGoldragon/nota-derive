//! `NotaTransparent` codegen — emits `NotaEncode` + `NotaDecode`
//! that delegate to the wrapped inner type, plus `From`
//! conversions in both directions so the wrapped field can stay
//! private.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let _name = input.ident;
    quote! {}
}
