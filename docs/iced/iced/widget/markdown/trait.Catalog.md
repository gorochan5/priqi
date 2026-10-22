---
title: "Catalog"
kind: trait
path: "iced::widget::markdown::Catalog"
crate: "iced"
module: "iced::widget::markdown"
file: "iced/widget/markdown/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/trait.Catalog.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "2305-2313"
---

# `iced::widget::markdown::Catalog`

```rust
pub trait Catalog:
    Catalog
    + Catalog
    + Catalog
    + Catalog
    + Catalog
    + Catalog
    + Clone
    + PartialEq {
    // Required methods
    fn id(&self) -> &str;
    fn link_color(&self) -> Color;
    fn code(&self) -> InlineCode;
    fn code_block<'a>() -> Self::Class<'a>;
    fn quote<'a>() -> Self::Class<'a>;
    fn highlighter(&self) -> &dyn Highlighter<Code, Self>;
}
```

The theme catalog of Markdown items.

**Source:** [markdown.rs](../../../src/iced_widget/markdown.rs.md) (lines 2305-2313)

## Required Methods

### `id`

```rust
fn id(&self) -> &str
```

The unique identifier of the [`Catalog`](trait.Catalog.md).

This will be used to invalidate span styling when a theme changes.

### `link_color`

```rust
fn link_color(&self) -> Color
```

The [`Color`](../../struct.Color.md) of some link.

### `code`

```rust
fn code(&self) -> InlineCode
```

The [`InlineCode`](struct.InlineCode.md) style of some inline code.

### `code_block`

```rust
fn code_block<'a>() -> Self::Class<'a>
```

The styling class of a code block.

### `quote`

```rust
fn quote<'a>() -> Self::Class<'a>
```

The styling class of a quote.

### `highlighter`

```rust
fn highlighter(&self) -> &dyn Highlighter<Code, Self>
```

The default [`text::Highlighter`](../../advanced/text/trait.Highlighter.md) to use to highlight code.

## Implementors

- `impl Catalog for Theme`
