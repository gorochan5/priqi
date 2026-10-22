---
title: "Catalog"
kind: trait
path: "iced_widget::toggler::Catalog"
crate: "iced_widget"
module: "iced_widget::toggler"
file: "iced_widget/toggler/trait.Catalog.md"
docs_url: "https://docs.iced.rs/iced_widget/toggler/trait.Catalog.html"
source_file: "src/iced_widget/toggler.rs"
source_lines: "523-535"
---

# `iced_widget::toggler::Catalog`

```rust
pub trait Catalog: Sized {
    type Class<'a>;

    // Required methods
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
    fn selection(&self) -> Color;
}
```

The theme catalog of a [`Toggler`](struct.Toggler.md).

**Source:** [toggler.rs](../../src/iced_widget/toggler.rs.md) (lines 523-535)

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

### `selection`

```rust
fn selection(&self) -> Color
```

The global selection [`Color`].

## Implementors

- `impl Catalog for Theme`
  - `type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>`
