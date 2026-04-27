//! `NotaRecord` codegen — emits `NotaEncode` + `NotaDecode`
//! impls that round-trip a struct as `(TypeName field0 field1 …)`.
//!
//! Empty structs (`pub struct Ok {}` or `pub struct Ok;`)
//! emit / accept just `(Ok)`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn expand(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let name_string = name.to_string();

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(named), .. }) => {
            named.named.iter().collect::<Vec<_>>()
        }
        Data::Struct(DataStruct { fields: Fields::Unit, .. }) => Vec::new(),
        Data::Struct(DataStruct { fields: Fields::Unnamed(_), .. }) => panic!(
            "NotaRecord requires a struct with named fields or a unit struct; for tuple-struct newtypes use NotaTransparent"
        ),
        _ => panic!("NotaRecord can only be derived for structs"),
    };

    let encode_field_calls = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("named field");
        quote! {
            self.#field_ident.encode(encoder)?;
        }
    });

    let decode_field_bindings = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("named field");
        let field_type = &field.ty;
        quote! {
            let #field_ident = <#field_type as ::nota_codec::NotaDecode>::decode(decoder)?;
        }
    });

    let init_field_idents = fields.iter().map(|field| field.ident.clone());

    let init_expr = if fields.is_empty() {
        quote! { Self {} }
    } else {
        quote! { Self { #(#init_field_idents),* } }
    };

    quote! {
        impl ::nota_codec::NotaEncode for #name {
            fn encode(&self, encoder: &mut ::nota_codec::Encoder) -> ::nota_codec::Result<()> {
                encoder.start_record(#name_string)?;
                #(#encode_field_calls)*
                encoder.end_record()
            }
        }

        impl ::nota_codec::NotaDecode for #name {
            fn decode(decoder: &mut ::nota_codec::Decoder<'_>) -> ::nota_codec::Result<Self> {
                decoder.expect_record_head(#name_string)?;
                #(#decode_field_bindings)*
                decoder.expect_record_end()?;
                Ok(#init_expr)
            }
        }
    }
}
