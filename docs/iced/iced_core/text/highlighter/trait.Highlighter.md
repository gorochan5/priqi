---
title: "Highlighter"
kind: trait
path: "iced_core::text::highlighter::Highlighter"
crate: "iced_core"
module: "iced_core::text::highlighter"
file: "iced_core/text/highlighter/trait.Highlighter.md"
docs_url: "https://docs.iced.rs/iced_core/text/highlighter/trait.Highlighter.html"
source_file: "src/iced_core/text/highlighter.rs"
source_lines: "7-13"
---

# `iced_core::text::highlighter::Highlighter`

```rust
pub trait Highlighter<Input, Theme = Theme> {
    // Required methods
    fn id(&self) -> &str;
    fn highlight(&self, input: Input, theme: &Theme) -> Style;
}
```

A type that describes how to highlight an `Input` with some [`Style`](struct.Style.md).

**Source:** [highlighter.rs](../../../src/iced_core/text/highlighter.rs.md) (lines 7-13)

## Required Methods

### `id`

```rust
fn id(&self) -> &str
```

A unique identifier for the highlighter.

### `highlight`

```rust
fn highlight(&self, input: Input, theme: &Theme) -> Style
```

Returns the [`Style`](struct.Style.md) of the given `Input`.

## Implementors

- `impl<T, Input, Theme> Highlighter<Input, Theme> for T where T: Fn(Input, &Theme) -> Style,`
