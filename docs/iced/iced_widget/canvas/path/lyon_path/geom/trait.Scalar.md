---
title: "Scalar"
kind: trait
path: "iced_widget::canvas::path::lyon_path::geom::Scalar"
crate: "iced_widget"
module: "iced_widget::canvas::path::lyon_path::geom"
file: "iced_widget/canvas/path/lyon_path/geom/trait.Scalar.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/path/lyon_path/geom/trait.Scalar.html"
---

# `iced_widget::canvas::path::lyon_path::geom::Scalar`

```rust
pub trait Scalar:
    Sized
    + Float
    + NumCast
    + FloatConst
    + Display
    + Debug
    + Trig
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign {
Show 16 associated constants and 2 methods    const HALF: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const FIVE: Self;
    const SIX: Self;
    const SEVEN: Self;
    const EIGHT: Self;
    const NINE: Self;
    const TEN: Self;
    const MIN: Self;
    const MAX: Self;
    const EPSILON: Self;
    const DIV_EPSILON: Self = Self::EPSILON;

    // Required method
    fn value(v: f32) -> Self;

    // Provided method
    fn epsilon_for(_reference: Self) -> Self { ... }
}
```

## Required Methods

### `value`

```rust
fn value(v: f32) -> Self
```

## Provided Methods

### `epsilon_for`

```rust
fn epsilon_for(_reference: Self) -> Self
```

Epsilon constants are usually not a good way to deal with float precision. Float precision depends on the magnitude of the values and so should appropriate epsilons.

## Implementors
