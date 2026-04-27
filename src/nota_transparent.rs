//! `NotaTransparent` codegen — emits `NotaEncode` + `NotaDecode`
//! that delegate to the wrapped inner type, plus `From`
//! conversions in both directions so the wrapped field can stay
//! private.

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
            "NotaTransparent requires a tuple struct with exactly one field, e.g. `pub struct Slot(u64);`"
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
                Ok(Self(inner))
            }
        }

        impl ::core::convert::From<#inner_type> for #name {
            fn from(inner: #inner_type) -> Self {
                Self(inner)
            }
        }

        impl ::core::convert::From<#name> for #inner_type {
            fn from(wrapper: #name) -> Self {
                wrapper.0
            }
        }
    }
}
