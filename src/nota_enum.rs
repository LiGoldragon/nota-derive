//! `NotaEnum` codegen — emits `NotaEncode` + `NotaDecode`
//! impls that round-trip a unit-variant enum as its
//! PascalCase variant identifier.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields};

pub fn expand(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let name_string = name.to_string();

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("NotaEnum can only be derived for enums (with unit variants only)"),
    };

    for variant in variants {
        if !matches!(variant.fields, Fields::Unit) {
            panic!(
                "NotaEnum requires every variant to be a unit variant; `{}::{}` carries data",
                name, variant.ident
            );
        }
    }

    let encode_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_string = variant_ident.to_string();
        quote! {
            Self::#variant_ident => encoder.write_pascal_identifier(#variant_string),
        }
    });

    let decode_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_string = variant_ident.to_string();
        quote! {
            #variant_string => Ok(Self::#variant_ident),
        }
    });

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
                let identifier = decoder.read_pascal_identifier()?;
                match identifier.as_str() {
                    #(#decode_arms)*
                    other => Err(::nota_codec::Error::UnknownVariant {
                        enum_name: #name_string,
                        got: other.to_string(),
                    }),
                }
            }
        }
    }
}
