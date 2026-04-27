//! `NexusVerb` codegen — emits `NotaEncode` + `NotaDecode` for
//! a closed enum whose variants name the kinds the verb
//! operates on.
//!
//! Two variant shapes are supported:
//!
//! - **Newtype**: `Node(Node)` — encodes as the payload's
//!   regular `NotaRecord` form (the variant ident matches the
//!   payload's record head).
//! - **Struct**: `Node { slot: Slot, new: Node, expected_rev: Option<Revision> }`
//!   — encodes as `(VariantName field0 field1 …)` with the
//!   variant ident as the head and fields written positionally.
//!
//! Decoding peeks the next record's head identifier and
//! dispatches to the matching variant.

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
                // Newtype variant — payload's NotaRecord encode
                // already emits `(VariantName ...)` because the
                // payload type's name matches the variant name
                // (caller convention).
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
            Fields::Named(named) => {
                let field_idents: Vec<_> = named
                    .named
                    .iter()
                    .map(|field| field.ident.clone().expect("named field"))
                    .collect();
                let field_types: Vec<_> = named.named.iter().map(|field| field.ty.clone()).collect();

                let encode_field_calls = field_idents.iter().map(|field_ident| {
                    quote! { #field_ident.encode(encoder)?; }
                });
                let decode_field_bindings =
                    field_idents.iter().zip(field_types.iter()).map(|(field_ident, field_type)| {
                        quote! {
                            let #field_ident = <#field_type as ::nota_codec::NotaDecode>::decode(decoder)?;
                        }
                    });

                encode_arms.push(quote! {
                    Self::#variant_ident { #(#field_idents),* } => {
                        encoder.start_record(#variant_string)?;
                        #(#encode_field_calls)*
                        encoder.end_record()
                    }
                });
                decode_arms.push(quote! {
                    #variant_string => {
                        decoder.expect_record_head(#variant_string)?;
                        #(#decode_field_bindings)*
                        decoder.expect_record_end()?;
                        Ok(Self::#variant_ident { #(#field_idents),* })
                    }
                });
            }
            Fields::Unit => panic!(
                "NexusVerb does not support unit variants; `{}::{}` carries no data — use NotaEnum for unit-variant enums",
                name, variant_ident
            ),
            _ => panic!(
                "NexusVerb requires every variant to be a newtype variant or a struct-variant; `{}::{}` has a different shape",
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
