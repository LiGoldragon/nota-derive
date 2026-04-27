//! `NotaRecord` codegen — emits `NotaEncode` + `NotaDecode`
//! impls that round-trip a struct as `(TypeName field0 field1 …)`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> TokenStream {
    let _name = input.ident;
    // Implementation lands in the next pass; for now emit nothing
    // so the derive-call site compiles even if the resulting type
    // doesn't yet implement the traits.
    quote! {}
}
