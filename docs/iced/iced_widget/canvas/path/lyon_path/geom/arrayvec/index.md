---
title: "arrayvec"
kind: module
path: "iced_widget::canvas::path::lyon_path::geom::arrayvec"
crate: "iced_widget"
module: "iced_widget::canvas::path::lyon_path::geom"
file: "iced_widget/canvas/path/lyon_path/geom/arrayvec/index.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/path/lyon_path/geom/arrayvec/index.html"
---

# `iced_widget::canvas::path::lyon_path::geom::arrayvec`

**arrayvec** provides the types [`ArrayVec`](struct.ArrayVec.md) and [`ArrayString`](struct.ArrayString.md): array-backed vector and string types, which store their contents inline.

The arrayvec package has the following cargo features:

- `std`

  - Optional, enabled by default
  - Use libstd; disable to use `no_std` instead.
- `serde`

  - Optional
  - Enable serialization for ArrayVec and ArrayString using serde 1.x
- `zeroize`

  - Optional
  - Implement `Zeroize` for ArrayVec and ArrayString

### Rust Version

This version of arrayvec requires Rust 1.51 or later.

## Structs

- [`ArrayString`](struct.ArrayString.md) — A string with a fixed capacity.
- [`ArrayVec`](struct.ArrayVec.md) — A vector with a fixed capacity.
- [`CapacityError`](struct.CapacityError.md) — Error value indicating insufficient capacity
- [`Drain`](struct.Drain.md) — A draining iterator for `ArrayVec`.
- [`IntoIter`](struct.IntoIter.md) — By-value iterator for `ArrayVec`.
