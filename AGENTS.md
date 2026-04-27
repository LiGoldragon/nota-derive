# Agent instructions

Workspace-wide rules + tools-documentation pointers live in
[`mentci/AGENTS.md`](https://github.com/LiGoldragon/mentci/blob/main/AGENTS.md).

This crate is the proc-macro half of the
nota-codec / nota-derive pair. Its job is to emit `NotaEncode`
+ `NotaDecode` impls for user types. Read [`ARCHITECTURE.md`](ARCHITECTURE.md)
before editing.

## Specific rules for this crate

- The emitted code references the runtime via fully-qualified
  paths (`::nota_codec::…`). **Do not** require the user to
  import anything for `#[derive(...)]` to compile.
- Per-derive logic lives in its own file. Adding a new derive
  means adding a new file under `src/`, a new entry point in
  `lib.rs`, and trybuild tests under `tests/compile_fail/`.
- Compile-fail tests are mandatory for every error path the
  derive can produce. The error message is part of the API.
- Do not author macros that generate macros (no
  meta-proc-macros). Each derive is a single proc-macro
  function.
