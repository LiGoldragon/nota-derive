//! `NexusVerb` codegen — emits `NotaEncode` + `NotaDecode` for
//! a closed enum whose variants name the kinds the verb
//! operates on. Decoding peeks the next record's head
//! identifier and dispatches to the matching variant.
//!
//! Supports newtype variants (`Node(Node)`); struct-variant
//! support lands in a follow-up pass once a real call site
//! needs it (`MutateOperation`).

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields};

pub fn expand(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let name_string = name.to_string();

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("NexusVerb can only be derived for enums"),
    };

    let mut encode_arms = Vec::new();
    let mut decode_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_string = variant_ident.to_string();
        match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let payload_type = &unnamed.unnamed.first().unwrap().ty;
                encode_arms.push(quote! {
                    Self::#variant_ident(value) => value.encode(encoder),
                });
                decode_arms.push(quote! {
                    #variant_string => Ok(Self::#variant_ident(
                        <#payload_type as ::nota_codec::NotaDecode>::decode(decoder)?
                    )),
                });
            }
            _ => panic!(
                "NexusVerb requires every variant to be a newtype variant carrying one payload type; `{}::{}` has a different shape (struct-variant support lands later)",
                name, variant_ident
            ),
        }
    }

    quote! {
        impl ::nota_codec::NotaEncode for #name {
            fn encode(&self, encoder: &mut ::nota_codec::Encoder) -> ::nota_codec::Result<()> {
                match self {
                    #(#encode_arms)*
                }
            }
        }

        impl ::nota_codec::NotaDecode for #name {
            fn decode(decoder: &mut ::nota_codec::Decoder<'_>) -> ::nota_codec::Result<Self> {
                let head = decoder.peek_record_head()?;
                match head.as_str() {
                    #(#decode_arms)*
                    other => Err(::nota_codec::Error::UnknownKindForVerb {
                        verb: #name_string,
                        got: other.to_string(),
                    }),
                }
            }
        }
    }
}
