//! Shared introspection helpers used by every derive.
//!
//! Field- and variant-shape inspection sits here so each per-
//! derive module can stay focused on its own codegen.

#![allow(dead_code)] // populated as each derive lands

use syn::{Data, DataStruct, Fields, FieldsNamed, FieldsUnnamed};

/// Extract the named fields of a struct, or panic with a
/// human-readable error if the input isn't a named-field struct.
pub fn named_fields(data: &Data, derive_name: &str) -> Option<FieldsNamed> {
    match data {
        Data::Struct(DataStruct { fields: Fields::Named(named), .. }) => Some(named.clone()),
        Data::Struct(DataStruct { fields: Fields::Unit, .. }) => None,
        Data::Struct(DataStruct { fields: Fields::Unnamed(_), .. }) => panic!(
            "{derive_name} requires a struct with named fields or a unit struct"
        ),
        _ => panic!("{derive_name} can only be derived for structs"),
    }
}

/// Extract the single tuple field of a unit-newtype, or panic.
pub fn single_unnamed_field(data: &Data, derive_name: &str) -> FieldsUnnamed {
    match data {
        Data::Struct(DataStruct { fields: Fields::Unnamed(unnamed), .. })
            if unnamed.unnamed.len() == 1 =>
        {
            unnamed.clone()
        }
        _ => panic!("{derive_name} requires a tuple struct with exactly one field"),
    }
}
