---
title: "Renderer"
kind: trait
path: "iced_core::renderer::Renderer"
crate: "iced_core"
module: "iced_core::renderer"
file: "iced_core/renderer/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_core/renderer/trait.Renderer.html"
source_file: "src/iced_core/renderer.rs"
source_lines: "16-97"
---

# `iced_core::renderer::Renderer`

```rust
pub trait Renderer: 'static {
Show 15 methods    // Required methods
    fn start_layer(&mut self, bounds: Rectangle);
    fn end_layer(&mut self);
    fn start_transformation(&mut self, transformation: Transformation);
    fn end_transformation(&mut self);
    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>);
    fn allocate_image(
        &self,
        handle: &Handle,
        callback: impl FnOnce(Result<Allocation, Error>) + Send + 'static,
    );
    fn hint(&mut self, scale: Scale);
    fn scale(&self) -> Option<Scale>;
    fn reset(&mut self, new_bounds: Rectangle);
    fn settings(&self) -> Settings;

    // Provided methods
    fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self)) { ... }
    fn with_transformation(
        &mut self,
        transformation: Transformation,
        f: impl FnOnce(&mut Self),
    ) { ... }
    fn with_translation(
        &mut self,
        translation: Vector,
        f: impl FnOnce(&mut Self),
    ) { ... }
    fn hint_factor(&self) -> Option<f32> { ... }
    fn tick(&mut self) { ... }
}
```

A component that can be used by widgets to draw themselves on a screen.

**Source:** [renderer.rs](../../src/iced_core/renderer.rs.md) (lines 16-97)

## Required Methods

### `start_layer`

```rust
fn start_layer(&mut self, bounds: Rectangle)
```

Starts recording a new layer.

### `end_layer`

```rust
fn end_layer(&mut self)
```

Ends recording a new layer.

The new layer will clip its contents to the provided `bounds`.

### `start_transformation`

```rust
fn start_transformation(&mut self, transformation: Transformation)
```

Starts recording with a new [`Transformation`](../struct.Transformation.md).

### `end_transformation`

```rust
fn end_transformation(&mut self)
```

Ends recording a new layer.

The new layer will clip its contents to the provided `bounds`.

### `fill_quad`

```rust
fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>)
```

Fills a [`Quad`](struct.Quad.md) with the provided [`Background`](../enum.Background.md).

### `allocate_image`

```rust
fn allocate_image( &self, handle: &Handle, callback: impl FnOnce(Result<Allocation, Error>) + Send + 'static, )
```

Creates an [`image::Allocation`](../image/struct.Allocation.md) for the given [`image::Handle`](../image/enum.Handle.md) and calls the given callback with it.

### `hint`

```rust
fn hint(&mut self, scale: Scale)
```

Provides hints to the [`Renderer`](trait.Renderer.md) about the rendering target.

This may be used internally by the [`Renderer`](trait.Renderer.md) to perform optimizations and/or improve rendering quality.

For instance, providing a [`Scale`](struct.Scale.md) may be used by some renderers to perform metrics hinting internally in physical coordinates while keeping layout coordinates logical and, therefore, maintain linearity.

### `scale`

```rust
fn scale(&self) -> Option<Scale>
```

Returns the last [`Scale`](struct.Scale.md) provided as a [`hint`](trait.Renderer.md).

### `reset`

```rust
fn reset(&mut self, new_bounds: Rectangle)
```

Resets the [`Renderer`](trait.Renderer.md) to start drawing in the `new_bounds` from scratch.

### `settings`

```rust
fn settings(&self) -> Settings
```

Returns the [`Settings`](struct.Settings.md) of this [`Renderer`](trait.Renderer.md).

## Provided Methods

### `with_layer`

```rust
fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self))
```

Draws the primitives recorded in the given closure in a new layer.

The layer will clip its contents to the provided `bounds`.

### `with_transformation`

```rust
fn with_transformation( &mut self, transformation: Transformation, f: impl FnOnce(&mut Self), )
```

Applies a [`Transformation`](../struct.Transformation.md) to the primitives recorded in the given closure.

### `with_translation`

```rust
fn with_translation(&mut self, translation: Vector, f: impl FnOnce(&mut Self))
```

Applies a translation to the primitives recorded in the given closure.

### `hint_factor`

```rust
fn hint_factor(&self) -> Option<f32>
```

Returns the last hint factor provided as a [`hint`](trait.Renderer.md), only if [`Settings::metrics_hinting`](struct.Settings.md) is enabled.

### `tick`

```rust
fn tick(&mut self)
```

Polls any concurrent computations that may be pending in the [`Renderer`](trait.Renderer.md).

By default, it does nothing.

## Implementors
