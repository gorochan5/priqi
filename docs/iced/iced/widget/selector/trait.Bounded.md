---
title: "Bounded"
kind: trait
path: "iced::widget::selector::Bounded"
crate: "iced"
module: "iced::widget::selector"
file: "iced/widget/selector/trait.Bounded.md"
docs_url: "https://docs.iced.rs/iced/widget/selector/trait.Bounded.html"
---

# `iced::widget::selector::Bounded`

```rust
pub trait Bounded: Debug {
    // Required methods
    fn bounds(&self) -> Rectangle;
    fn visible_bounds(&self) -> Option<Rectangle>;
}
```

A bounded type has both layout bounds and visible bounds.

This trait lets us write generic code over the [`Output`](trait.Selector.md) of a [`Selector`](trait.Selector.md).

## Required Methods

### `bounds`

```rust
fn bounds(&self) -> Rectangle
```

Returns the layout bounds.

### `visible_bounds`

```rust
fn visible_bounds(&self) -> Option<Rectangle>
```

Returns the visible bounds, in screen coordinates.

## Implementors

- `impl Bounded for Target`
- `impl Bounded for Text`
