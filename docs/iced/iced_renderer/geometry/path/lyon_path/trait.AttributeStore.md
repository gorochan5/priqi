---
title: "AttributeStore"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::AttributeStore"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/trait.AttributeStore.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/trait.AttributeStore.html"
---

# `iced_renderer::geometry::path::lyon_path::AttributeStore`

```rust
pub trait AttributeStore {
    // Required methods
    fn get(&self, id: EndpointId) -> &[f32];
    fn num_attributes(&self) -> usize;
}
```

Interface for objects storing custom attributes associated with endpoints.

This interface can be implemented by path objects themselves or via external data structures.

## Required Methods

### `get`

```rust
fn get(&self, id: EndpointId) -> &[f32]
```

Returns the endpoint’s custom attributes as a slice of 32 bits floats.

The size of the slice must be equal to the result of `num_attributes()`.

### `num_attributes`

```rust
fn num_attributes(&self) -> usize
```

Returns the number of float attributes per endpoint.

All endpoints must have the same number of attributes.

## Implementors

- `impl AttributeStore for Path`
- `impl<'l> AttributeStore for AttributeSlice<'l>`
- `impl<'l> AttributeStore for PathSlice<'l>`
