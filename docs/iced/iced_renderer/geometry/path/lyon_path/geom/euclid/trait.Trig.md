---
title: "Trig"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::Trig"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/trait.Trig.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/trait.Trig.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::Trig`

```rust
pub trait Trig {
    // Required methods
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn fast_atan2(y: Self, x: Self) -> Self;
    fn degrees_to_radians(deg: Self) -> Self;
    fn radians_to_degrees(rad: Self) -> Self;
}
```

Trait for basic trigonometry functions, so they can be used on generic numeric types

## Required Methods

### `sin`

```rust
fn sin(self) -> Self
```

### `cos`

```rust
fn cos(self) -> Self
```

### `tan`

```rust
fn tan(self) -> Self
```

### `fast_atan2`

```rust
fn fast_atan2(y: Self, x: Self) -> Self
```

### `degrees_to_radians`

```rust
fn degrees_to_radians(deg: Self) -> Self
```

### `radians_to_degrees`

```rust
fn radians_to_degrees(rad: Self) -> Self
```

## Implementors
