---
title: "Catalog"
kind: trait
path: "iced_widget::text_editor::Catalog"
crate: "iced_widget"
module: "iced_widget::text_editor"
file: "iced_widget/text_editor/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/text_editor/trait.Catalog.html"
source_file: "src/iced_widget/text_editor.rs"
source_lines: "769-778"
---

# `iced_widget::text_editor::Catalog`

```rust
pub trait Catalog: Base {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
```

The theme catalog of a [`TextEditor`](struct.TextEditor.md).

**Source:** [text_editor.rs](../../src/iced_widget/text_editor.rs.md) (lines 769-778)

## Required Associated Types

### `Class`

```rust
type Class<'a>
```

The item class of the [`Catalog`](trait.Catalog.md).

## Required Methods

### `default`

```rust
fn default<'a>() -> Self::Class<'a>
```

The default class produced by the [`Catalog`](trait.Catalog.md).

### `style`

```rust
fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
