---
title: "Float"
kind: trait
path: "iced::animation::Float"
crate: "iced"
module: "iced::animation"
file: "iced/animation/trait.Float.md"
docs_url: "https://docs.iced.rs/iced/animation/trait.Float.html"
---

# `iced::animation::Float`

```rust
pub trait Float {
    // Required method
    fn float_value(&self) -> f32;
}
```

Defines a float representation for arbitrary types

The actual float values are pretty arbitrary - as interpolation from one float to another will look the same, however in the case of asymmetric animations the ‘direction’ of the animation is determined using these float representations. In general, this defines ‘keyframes’ & associates animated values on a continuous axis so that transitions & interruptions can be represented.

## Required Methods

### `float_value`

```rust
fn float_value(&self) -> f32
```

## Implementors
