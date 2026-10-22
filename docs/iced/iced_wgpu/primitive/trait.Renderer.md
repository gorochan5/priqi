---
title: "Renderer"
kind: trait
path: "iced_wgpu::primitive::Renderer"
crate: "iced_wgpu"
module: "iced_wgpu::primitive"
file: "iced_wgpu/primitive/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_wgpu/primitive/trait.Renderer.html"
source_file: "src/iced_wgpu/primitive.rs"
source_lines: "182-185"
---

# `iced_wgpu::primitive::Renderer`

```rust
pub trait Renderer: Renderer {
    // Required method
    fn draw_primitive(&mut self, bounds: Rectangle, primitive: impl Primitive);
}
```

A renderer than can draw custom primitives.

**Source:** [primitive.rs](../../src/iced_wgpu/primitive.rs.md) (lines 182-185)

## Required Methods

### `draw_primitive`

```rust
fn draw_primitive(&mut self, bounds: Rectangle, primitive: impl Primitive)
```

Draws a custom primitive.

## Implementors

- `impl Renderer for Renderer`
