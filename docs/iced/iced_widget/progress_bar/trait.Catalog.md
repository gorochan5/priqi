---
title: "Catalog"
kind: trait
path: "iced_widget::progress_bar::Catalog"
crate: "iced_widget"
module: "iced_widget::progress_bar"
file: "iced_widget/progress_bar/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/progress_bar/trait.Catalog.html"
source_file: "src/iced_widget/progress_bar.rs"
source_lines: "237-246"
---

# `iced_widget::progress_bar::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
```

The theme catalog of a [`ProgressBar`](struct.ProgressBar.md).

**Source:** [progress_bar.rs](../../src/iced_widget/progress_bar.rs.md) (lines 237-246)

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
fn style(&self, class: &Self::Class<'_>) -> Style
```

The [`Style`](struct.Style.md) of a class with the given status.

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>`
