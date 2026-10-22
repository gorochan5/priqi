---
title: "Meta"
kind: trait
path: "iced_core::widget::Meta"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/trait.Meta.md"
docs_url: "https://docs.iced.rs/iced_core/widget/trait.Meta.html"
source_file: "src/iced_core/widget.rs"
source_lines: "161-166"
---

# `iced_core::widget::Meta`

```rust
pub trait Meta {
    // Provided method
    fn is_void(&self) -> bool { ... }
}
```

Metadata of a [`Widget`](trait.Widget.md), independent of its generic parameters.

**Source:** [widget.rs](../../src/iced_core/widget.rs.md) (lines 161-166)

## Provided Methods

### `is_void`

```rust
fn is_void(&self) -> bool
```

Returns whether the [`Widget`](trait.Widget.md) is [`Void`](struct.Void.md).

## Implementors

- `impl Meta for Void`
- `impl<'a, Message, Theme, Renderer> Meta for Element<'a, Message, Theme, Renderer>`
- `impl<Theme> Meta for Text<'_, Theme> where Theme: Catalog,`
- `impl<W, F, A> Meta for Map<W, F, A> where W: Meta,`
