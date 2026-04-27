//! `NexusPattern` codegen — emits `NotaEncode` + `NotaDecode`
//! that round-trip a `*Query` struct as nexus pattern-record
//! form `(| RecordName field0 field1 … |)`.
//!
//! The data-record name comes from the required
//! `#[nota(queries = "Name")]` attribute (the wire uses the
//! data record's name, not the query type's Rust name).
//!
//! Each field is a `PatternField<T>`. The field's schema name
//! (= the Rust field identifier) is passed to
//! `decode_pattern_field` so the bind-name validation can
//! check `@<name>` against the expected schema field name.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Expr, Fields, Lit, Meta};

pub fn expand(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let queries_record_name = parse_queries_attribute(&input.attrs).unwrap_or_else(|| {
        panic!(
            "NexusPattern requires #[nota(queries = \"RecordName\")] on the type to name the data record this query targets — `{}` is missing it",
            name
        )
    });

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(named), .. }) => {
            named.named.iter().collect::<Vec<_>>()
        }
        _ => panic!(
            "NexusPattern requires a struct with named fields whose types are `PatternField<T>`"
        ),
    };

    let encode_field_calls = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("named field");
        let field_string = field_ident.to_string();
        quote! {
            encoder.encode_pattern_field(&self.#field_ident, #field_string)?;
        }
    });

    let decode_field_bindings = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("named field");
        let field_string = field_ident.to_string();
        let field_type = &field.ty;
        // field_type is `PatternField<T>`; extract T.
        let inner_type = extract_pattern_field_inner(field_type).unwrap_or_else(|| {
            panic!(
                "NexusPattern requires every field to be `PatternField<T>`; `{}::{}` is not",
                name, field_ident
            )
        });
        quote! {
            let #field_ident = decoder.decode_pattern_field::<#inner_type>(#field_string)?;
        }
    });

    let init_field_idents = fields.iter().map(|field| field.ident.clone());

    quote! {
        impl ::nota_codec::NotaEncode for #name {
            fn encode(&self, encoder: &mut ::nota_codec::Encoder) -> ::nota_codec::Result<()> {
                encoder.start_pattern_record(#queries_record_name)?;
                #(#encode_field_calls)*
                encoder.end_pattern_record()
            }
        }

        impl ::nota_codec::NotaDecode for #name {
            fn decode(decoder: &mut ::nota_codec::Decoder<'_>) -> ::nota_codec::Result<Self> {
                decoder.expect_pattern_record_head(#queries_record_name)?;
                #(#decode_field_bindings)*
                decoder.expect_pattern_record_end()?;
                Ok(Self { #(#init_field_idents),* })
            }
        }
    }
}

/// Parse `#[nota(queries = "Foo")]` and return `"Foo"`.
fn parse_queries_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("nota") {
            continue;
        }
        let meta_list = match &attr.meta {
            Meta::List(list) => list,
            _ => continue,
        };
        let mut found: Option<String> = None;
        let _ = meta_list.parse_nested_meta(|nested| {
            if !nested.path.is_ident("queries") {
                return Ok(());
            }
            let value: Expr = nested.value()?.parse()?;
            if let Expr::Lit(expr_lit) = value {
                if let Lit::Str(literal) = expr_lit.lit {
                    found = Some(literal.value());
                }
            }
            Ok(())
        });
        if found.is_some() {
            return found;
        }
    }
    None
}

/// Given a `syn::Type` that should look like `PatternField<T>`,
/// return the `T`.
fn extract_pattern_field_inner(field_type: &syn::Type) -> Option<syn::Type> {
    let path = match field_type {
        syn::Type::Path(type_path) => &type_path.path,
        _ => return None,
    };
    let last_segment = path.segments.last()?;
    if last_segment.ident != "PatternField" {
        return None;
    }
    let args = match &last_segment.arguments {
        syn::PathArguments::AngleBracketed(args) => args,
        _ => return None,
    };
    let first_arg = args.args.first()?;
    match first_arg {
        syn::GenericArgument::Type(inner_type) => Some(inner_type.clone()),
        _ => None,
    }
}
