//! nota-derive — proc-macro derives for `nota-codec`.
//!
//! Five derives. All re-exported through `nota-codec` so users
//! depend on a single crate. See the per-derive modules for the
//! codegen logic; this file is just dispatch.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod nexus_pattern;
mod nexus_verb;
mod nota_enum;
mod nota_record;
mod nota_transparent;
mod nota_try_transparent;
mod shared;

/// Derive `NotaEncode` + `NotaDecode` for a struct that
/// represents a nota-text record. Encodes as `(TypeName field0
/// field1 …)`; decodes from the same shape.
///
/// The struct's fields must implement `NotaEncode` +
/// `NotaDecode`. Optional fields (`Option<T>`) may appear only
/// at the end of the struct; the wire form omits them when
/// `None`.
#[proc_macro_derive(NotaRecord)]
pub fn derive_nota_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nota_record::expand(input).into()
}

/// Derive `NotaEncode` + `NotaDecode` for a unit-variant enum.
/// Each variant encodes as its PascalCase identifier
/// (`Flow`, `DependsOn`, …).
#[proc_macro_derive(NotaEnum)]
pub fn derive_nota_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nota_enum::expand(input).into()
}

/// Derive `NotaEncode` + `NotaDecode` for a tuple-struct
/// newtype wrapping a single primitive (`u64`, `[u8; N]`,
/// `String`, …). The encoded form is the inner value bare
/// (`Slot(42)` → `42`); the type wrapper is invisible at the
/// wire boundary.
///
/// Also emits `From<Inner> for Self` and `From<Self> for Inner`
/// so the wrapped field can stay private without callers
/// needing direct field access.
///
/// For newtypes whose construction is **fallible** (e.g.
/// `SshPubKey(String)` that validates ed25519 base64), use
/// [`NotaTryTransparent`] instead.
#[proc_macro_derive(NotaTransparent)]
pub fn derive_nota_transparent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nota_transparent::expand(input).into()
}

/// Derive `NotaEncode` + `NotaDecode` for a tuple-struct
/// newtype whose construction is **fallible** — the type
/// must expose `fn try_new(inner: Inner) -> Result<Self, E>`
/// where `E: ::core::fmt::Display`. The decoder runs
/// `try_new` after parsing the inner value; failures become
/// [`nota_codec::Error::Validation`] carrying the type name
/// + the validator's error message.
///
/// Use this for newtypes that enforce format constraints
/// (`SshPubKey` ed25519 base64, `Ipv6Addr`-parseable strings,
/// hex digests of fixed length, …) so that the validator
/// runs on every decode and malformed wire data fails at the
/// codec boundary rather than at first use.
///
/// Emits `From<Self> for Inner` (read-out is infallible) but
/// NOT `From<Inner> for Self` (construction goes through
/// `try_new`).
#[proc_macro_derive(NotaTryTransparent)]
pub fn derive_nota_try_transparent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nota_try_transparent::expand(input).into()
}

/// Derive `NotaEncode` + `NotaDecode` for a `*Query` struct
/// whose fields are `PatternField<T>`. Encodes as the nexus-
/// only pattern-record form: `(| RecordName field0 field1 … |)`.
///
/// Requires a `#[nota(queries = "RecordName")]` attribute that
/// names the data record this query targets — the wire form
/// uses the data record's name, not the query type's name.
#[proc_macro_derive(NexusPattern, attributes(nota))]
pub fn derive_nexus_pattern(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nexus_pattern::expand(input).into()
}

/// Derive `NotaEncode` + `NotaDecode` for a closed enum whose
/// variants name the kinds the verb operates on. Decoding
/// peeks the head identifier of the next record and dispatches
/// to the matching variant; encoding delegates straight to the
/// variant's payload.
///
/// Variants may be newtype (`Node(Node)`) or struct
/// (`Node { slot: Slot, new: Node, expected_rev: Option<Revision> }`);
/// struct variants encode positionally.
#[proc_macro_derive(NexusVerb)]
pub fn derive_nexus_verb(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nexus_verb::expand(input).into()
}
