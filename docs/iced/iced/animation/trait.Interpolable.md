---
title: "Interpolable"
kind: trait
path: "iced::animation::Interpolable"
crate: "iced"
module: "iced::animation"
file: "iced/animation/trait.Interpolable.md"
docs_url: "https://docs.iced.rs/iced/animation/trait.Interpolable.html"
---

# `iced::animation::Interpolable`

```rust
pub trait Interpolable {
    // Required method
    fn interpolated(&self, other: Self, ratio: f32) -> Self;
}
```

A type implementing `Interpolable` can be used with `Animated<T>.animate(...)`

## Required Methods

### `interpolated`

```rust
fn interpolated(&self, other: Self, ratio: f32) -> Self
```

## Implementors

- `impl Interpolable for Color`
