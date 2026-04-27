# nota-derive

Proc-macro derives for [`nota-codec`](https://github.com/LiGoldragon/nota-codec).
Five derives, all re-exported through `nota-codec` so users only
depend on the runtime crate.

| Derive | Dialect | Emits |
|---|---|---|
| `#[derive(NotaRecord)]` | both | `(Foo a b c)` form |
| `#[derive(NotaEnum)]` | both | unit-variant enum dispatched on PascalCase identifier |
| `#[derive(NotaTransparent)]` | both | newtype-of-primitive — emits the inner value bare (`Slot(42)` → `42`) |
| `#[derive(NexusPattern)]` | nexus-only | `(\| Foo a b c \|)` form with `PatternField<T>` semantics |
| `#[derive(NexusVerb)]` | nexus-only | closed-kind enum dispatched on head identifier |

The derives emit impls of `nota_codec::NotaEncode` and
`nota_codec::NotaDecode`. They reference the runtime via fully-
qualified paths (`::nota_codec::…`), so users do not need to
import the traits to use the derives.

Design + rationale: see
[mentci/reports/099](https://github.com/LiGoldragon/mentci/blob/main/reports/099-custom-derive-design-2026-04-27.md).

## License

[License of Non-Authority](LICENSE.md).
