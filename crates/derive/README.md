# derive

Internal derive macros for [errai](../../README.md).

`#[derive(Frame)]` generates the `Decode` impl, the `IntoOwned` impl and field accessors for each frame content type in `crates/parser`. The generated code refers to `crate::` paths and only compiles inside the parser crate.
