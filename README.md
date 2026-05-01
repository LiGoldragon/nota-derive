# nota-derive

Proc-macro derives for `nota-codec`.
Six derives, all re-exported through `nota-codec` so users only
depend on the runtime crate.

| Derive | Dialect | Emits |
|---|---|---|
| `#[derive(NotaRecord)]` | both | `(Foo a b c)` form |
| `#[derive(NotaEnum)]` | both | unit-variant enum dispatched on PascalCase identifier |
| `#[derive(NotaTransparent)]` | both | newtype-of-primitive — emits the inner value bare (`Slot(42)` → `42`) |
| `#[derive(NotaTryTransparent)]` | both | fallible newtype — same wire form as `NotaTransparent`, but `decode` returns `Result` for validating constructors |
| `#[derive(NexusPattern)]` | nexus-only | `(\| Foo a b c \|)` form with `PatternField<T>` semantics |
| `#[derive(NexusVerb)]` | nexus-only | closed-kind enum dispatched on head identifier |

The derives emit impls of `nota_codec::NotaEncode` and
`nota_codec::NotaDecode`. They reference the runtime via fully-
qualified paths (`::nota_codec::…`), so users do not need to
import the traits to use the derives.

## License

[License of Non-Authority](LICENSE.md).
