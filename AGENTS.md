# Agent instructions — nota-derive

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

The **proc-macro half** of the nota-codec / nota-derive pair. Emits `NotaEncode` + `NotaDecode` impls for user types.

---

## Carve-outs worth knowing

- The emitted code references the runtime via fully-qualified paths (`::nota_codec::…`). **Do not** require the user to import anything for `#[derive(...)]` to compile.
- Per-derive logic lives in its own file. Adding a new derive means adding a new file under `src/`, a new entry point in `lib.rs`, and trybuild tests under `tests/compile_fail/`.
- Compile-fail tests are mandatory for every error path the derive can produce. The error message is part of the API.
- Do not author macros that generate macros (no meta-proc-macros). Each derive is a single proc-macro function.
