# nota-derive — architecture

## Role

Proc-macro derives for [`nota-codec`](https://github.com/LiGoldragon/nota-codec).
This crate is **only** the proc-macro logic; the runtime
(traits, `Decoder`, `Encoder`) lives in `nota-codec` and is
re-exported alongside these derives so users depend on a
single crate.

The split exists because Rust requires `proc-macro = true`
crates to be separate from regular library crates.

## Boundaries

**Owns:**
- The six `#[proc_macro_derive]` entry points listed in
  [README.md](README.md).
- The codegen logic that turns a `DeriveInput` into impls of
  `NotaEncode` + `NotaDecode`.
- trybuild-based compile-fail tests that verify malformed
  inputs are rejected with helpful errors.

**Does not own:**
- The traits `NotaEncode` / `NotaDecode` themselves — those
  live in `nota-codec`.
- The runtime methods on `Decoder` / `Encoder` that the
  emitted code calls into — also `nota-codec`.
- The `Lexer`, `Token` enum, or any tokenizer logic — also
  `nota-codec`.
- Round-trip integration tests that exercise both the
  derives and the runtime — those live in `nota-codec`'s
  `tests/` directory.

## Code map

```
src/
├── lib.rs                  # 6 #[proc_macro_derive] entry points
├── nota_record.rs          # NotaRecord codegen
├── nota_enum.rs            # NotaEnum codegen
├── nota_transparent.rs     # NotaTransparent codegen
├── nota_try_transparent.rs # NotaTryTransparent codegen (fallible newtype)
├── nexus_pattern.rs        # NexusPattern codegen (with bind-name validation)
├── nexus_verb.rs           # NexusVerb codegen (head-identifier dispatch)
└── shared.rs               # field/variant introspection helpers

tests/
└── compile_fail/         # trybuild compile-fail cases
```

## Cross-cutting context

- The derives target the trait + runtime API in
  [`nota-codec`](https://github.com/LiGoldragon/nota-codec/blob/main/ARCHITECTURE.md).
- Both crates exist as the typed text codec for the nota and
  nexus dialects — replacing the previous serde-based path with
  closed-enum dispatch.
- The derives align with criome's perfect-specificity invariant
  (closed enum dispatch, no string-tagged routing) — see
  [criome ARCHITECTURE.md §2 Invariant D](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d).

## Status

**v0.1** — skeleton in place. Each derive entry point is
stubbed; codegen lands incrementally as nota-codec's runtime
methods stabilise.
