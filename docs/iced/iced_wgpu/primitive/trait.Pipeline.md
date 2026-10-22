---
title: "Pipeline"
kind: trait
path: "iced_wgpu::primitive::Pipeline"
crate: "iced_wgpu"
module: "iced_wgpu::primitive"
file: "iced_wgpu/primitive/trait.Pipeline.md"
docs_url: "https://docs.iced.rs/iced_wgpu/primitive/trait.Pipeline.html"
source_file: "src/iced_wgpu/primitive.rs"
source_lines: "67-80"
---

# `iced_wgpu::primitive::Pipeline`

```rust
pub trait Pipeline:
    Any
    + MaybeSend
    + MaybeSync {
    // Required method
    fn new(device: &Device, queue: &Queue, format: TextureFormat) -> Self
       where Self: Sized;

    // Provided method
    fn trim(&mut self) { ... }
}
```

The pipeline of a graphics [`Primitive`](trait.Primitive.md).

**Source:** [primitive.rs](../../src/iced_wgpu/primitive.rs.md) (lines 67-80)

## Required Methods

### `new`

```rust
fn new(device: &Device, queue: &Queue, format: TextureFormat) -> Self where Self: Sized,
```

Creates the [`Pipeline`](trait.Pipeline.md) of a [`Primitive`](trait.Primitive.md).

This will only be called once, when the first [`Primitive`](trait.Primitive.md) with this kind of [`Pipeline`](trait.Pipeline.md) is encountered.

## Provided Methods

### `trim`

```rust
fn trim(&mut self)
```

Trims any cached data in the [`Pipeline`](trait.Pipeline.md).

This will normally be called at the end of a frame.

## Implementors
