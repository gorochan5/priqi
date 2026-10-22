---
title: "Primitive"
kind: trait
path: "iced_wgpu::primitive::Primitive"
crate: "iced_wgpu"
module: "iced_wgpu::primitive"
file: "iced_wgpu/primitive/trait.Primitive.md"
docs_url: "https://docs.iced.rs/iced_wgpu/primitive/trait.Primitive.html"
source_file: "src/iced_wgpu/primitive.rs"
source_lines: "14-64"
---

# `iced_wgpu::primitive::Primitive`

```rust
pub trait Primitive:
    Debug
    + MaybeSend
    + MaybeSync
    + 'static {
    type Pipeline: Pipeline + MaybeSend + MaybeSync;

    // Required method
    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &Device,
        queue: &Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    );

    // Provided methods
    fn draw(
        &self,
        _pipeline: &Self::Pipeline,
        _render_pass: &mut RenderPass<'_>,
    ) -> bool { ... }
    fn render(
        &self,
        _pipeline: &Self::Pipeline,
        _encoder: &mut CommandEncoder,
        _target: &TextureView,
        _clip_bounds: &Rectangle<u32>,
    ) { ... }
}
```

A set of methods which allows a [`Primitive`](trait.Primitive.md) to be rendered.

**Source:** [primitive.rs](../../src/iced_wgpu/primitive.rs.md) (lines 14-64)

## Required Associated Types

### `Pipeline`

```rust
type Pipeline: Pipeline + MaybeSend + MaybeSync
```

The shared renderer of this [`Primitive`](trait.Primitive.md).

Normally, this will contain a bunch of [`wgpu`] state; like a rendering pipeline, buffers, and textures.

All instances of this [`Primitive`](trait.Primitive.md) type will share the same [`Renderer`](trait.Renderer.md).

## Required Methods

### `prepare`

```rust
fn prepare( &self, pipeline: &mut Self::Pipeline, device: &Device, queue: &Queue, bounds: &Rectangle, viewport: &Viewport, )
```

Processes the [`Primitive`](trait.Primitive.md), allowing for GPU buffer allocation.

## Provided Methods

### `draw`

```rust
fn draw( &self, _pipeline: &Self::Pipeline, _render_pass: &mut RenderPass<'_>, ) -> bool
```

Draws the [`Primitive`](trait.Primitive.md) in the given [`wgpu::RenderPass`].

When possible, this should be implemented over [`render`](trait.Primitive.md) since reusing the existing render pass should be considerably more efficient than issuing a new one.

The viewport and scissor rect of the render pass provided is set to the bounds and clip bounds of the [`Primitive`](trait.Primitive.md), respectively.

If you have complex composition needs, then you can leverage [`render`](trait.Primitive.md) by returning `false` here.

By default, it does nothing and returns `false`.

### `render`

```rust
fn render( &self, _pipeline: &Self::Pipeline, _encoder: &mut CommandEncoder, _target: &TextureView, _clip_bounds: &Rectangle<u32>, )
```

Renders the [`Primitive`](trait.Primitive.md), using the given [`wgpu::CommandEncoder`].

This will only be called if [`draw`](trait.Primitive.md) returns `false`.

By default, it does nothing.

## Implementors
