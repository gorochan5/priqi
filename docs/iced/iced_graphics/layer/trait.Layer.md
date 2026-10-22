---
title: "Layer"
kind: trait
path: "iced_graphics::layer::Layer"
crate: "iced_graphics"
module: "iced_graphics::layer"
file: "iced_graphics/layer/trait.Layer.md"
docs_url: "https://docs.iced.rs/iced_graphics/layer/trait.Layer.html"
source_file: "src/iced_graphics/layer.rs"
source_lines: "8-44"
---

# `iced_graphics::layer::Layer`

```rust
pub trait Layer: Default {
    // Required methods
    fn with_bounds(bounds: Rectangle) -> Self;
    fn bounds(&self) -> Rectangle;
    fn flush(&mut self);
    fn resize(&mut self, bounds: Rectangle);
    fn reset(&mut self);
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn merge(&mut self, _layer: &mut Self);
}
```

A layer of graphical primitives.

Layers normally dictate a set of primitives that are rendered in a specific order.

**Source:** [layer.rs](../../src/iced_graphics/layer.rs.md) (lines 8-44)

## Required Methods

### `with_bounds`

```rust
fn with_bounds(bounds: Rectangle) -> Self
```

Creates a new [`Layer`](trait.Layer.md) with the given bounds.

### `bounds`

```rust
fn bounds(&self) -> Rectangle
```

Returns the current bounds of the [`Layer`](trait.Layer.md).

### `flush`

```rust
fn flush(&mut self)
```

Flushes and settles any pending group of primitives in the [`Layer`](trait.Layer.md).

This will be called when a [`Layer`](trait.Layer.md) is finished. It allows layers to efficiently record primitives together and defer grouping until the end.

### `resize`

```rust
fn resize(&mut self, bounds: Rectangle)
```

Resizes the [`Layer`](trait.Layer.md) to the given bounds.

### `reset`

```rust
fn reset(&mut self)
```

Clears all the layers contents and resets its bounds.

### `start`

```rust
fn start(&self) -> usize
```

Returns the start level of the [`Layer`](trait.Layer.md).

A level is a “sublayer” index inside of a [`Layer`](trait.Layer.md).

A [`Layer`](trait.Layer.md) may draw multiple primitive types in a certain order. The level represents the lowest index of the primitive types it contains.

Two layers A and B can therefore be merged if they have the same bounds, and the end level of A is lower or equal than the start level of B.

### `end`

```rust
fn end(&self) -> usize
```

Returns the end level of the [`Layer`](trait.Layer.md).

### `merge`

```rust
fn merge(&mut self, _layer: &mut Self)
```

Merges a [`Layer`](trait.Layer.md) with the current one.

## Implementors
