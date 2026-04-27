//! `NotaTryTransparent` codegen — like `NotaTransparent` but
//! routes decode through the user's `Self::try_new(inner) ->
//! Result<Self, _>` validator. Used for newtypes that wrap a
//! primitive but enforce a format constraint (`SshPubKey`
//! ed25519 base64, `Ipv6Addr`-parseable strings, hex digests
//! of fixed length, etc).
//!
//! Encoder side delegates to the inner type (same as
//! `NotaTransparent`); decoder runs `Self::try_new` and maps
//! the user's error type into `nota_codec::Error::Validation`
//! via `Display`.
//!
//! Construction goes through `Self::try_new` (fallible), so
//! no `From<Inner> for Self` is emitted. `From<Self> for Inner`
//! is still emitted since read-out is infallible.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn expand(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let inner_type = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Unnamed(unnamed), .. })
            if unnamed.unnamed.len() == 1 =>
        {
            unnamed.unnamed.first().unwrap().ty.clone()
        }
        _ => panic!(
            "NotaTryTransparent requires a tuple struct with exactly one field, e.g. `pub struct SshPubKey(String);`"
        ),
    };

    quote! {
        impl ::nota_codec::NotaEncode for #name {
            fn encode(&self, encoder: &mut ::nota_codec::Encoder) -> ::nota_codec::Result<()> {
                <#inner_type as ::nota_codec::NotaEncode>::encode(&self.0, encoder)
            }
        }

        impl ::nota_codec::NotaDecode for #name {
            fn decode(decoder: &mut ::nota_codec::Decoder<'_>) -> ::nota_codec::Result<Self> {
                let inner = <#inner_type as ::nota_codec::NotaDecode>::decode(decoder)?;
                Self::try_new(inner).map_err(|error| ::nota_codec::Error::Validation {
                    type_name: stringify!(#name),
                    message: ::std::format!("{error}"),
                })
            }
        }

        impl ::core::convert::From<#name> for #inner_type {
            fn from(wrapper: #name) -> Self {
                wrapper.0
            }
        }
    }
}
