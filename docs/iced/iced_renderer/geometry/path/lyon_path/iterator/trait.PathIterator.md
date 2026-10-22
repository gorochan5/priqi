---
title: "PathIterator"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::iterator::PathIterator"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::iterator"
file: "iced_renderer/geometry/path/lyon_path/iterator/trait.PathIterator.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/iterator/trait.PathIterator.html"
---

# `iced_renderer::geometry::path::lyon_path::iterator::PathIterator`

```rust
pub trait PathIterator: Sized + Iterator<Item = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>> {
    // Provided methods
    fn flattened(self, tolerance: f32) -> Flattened<Self> ⓘ { ... }
    fn transformed<T>(self, mat: &T) -> Transformed<'_, Self, T> ⓘ
       where T: Transformation<f32> { ... }
}
```

An extension trait for `PathEvent` iterators.

## Provided Methods

### `flattened`

```rust
fn flattened(self, tolerance: f32) -> Flattened<Self> ⓘ
```

Returns an iterator that turns curves into line segments.

### `transformed`

```rust
fn transformed<T>(self, mat: &T) -> Transformed<'_, Self, T> ⓘ where T: Transformation<f32>,
```

Returns an iterator applying a 2D transform to all of its events.

## Implementors

- `impl<Iter> PathIterator for Iter where Iter: Iterator<Item = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>>,`
