---
title: "Renderer"
kind: trait
path: "iced_core::image::Renderer"
crate: "iced_core"
module: "iced_core::image"
file: "iced_core/image/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_core/image/trait.Renderer.html"
source_file: "src/iced_core/image.rs"
source_lines: "285-313"
---

# `iced_core::image::Renderer`

```rust
pub trait Renderer: Renderer {
    type Handle: Clone;

    // Required methods
    fn load_image(&self, handle: &Self::Handle) -> Result<Allocation, Error>;
    fn measure_image(&self, handle: &Self::Handle) -> Option<Size<u32>>;
    fn draw_image(
        &mut self,
        image: Image<Self::Handle>,
        bounds: Rectangle,
        clip_bounds: Rectangle,
    );
}
```

A [`Renderer`](trait.Renderer.md) that can render raster graphics.

**Source:** [image.rs](../../src/iced_core/image.rs.md) (lines 285-313)

## Required Associated Types

### `Handle`

```rust
type Handle: Clone
```

The image Handle to be displayed. Iced exposes its own default implementation of a [`Handle`](trait.Renderer.md)

## Required Methods

### `load_image`

```rust
fn load_image(&self, handle: &Self::Handle) -> Result<Allocation, Error>
```

Loads an image and returns an explicit [`Allocation`](struct.Allocation.md) to it.

If the image is not already loaded, this method will block! You should generally not use it in drawing logic if you want to avoid frame drops.

### `measure_image`

```rust
fn measure_image(&self, handle: &Self::Handle) -> Option<Size<u32>>
```

Returns the dimensions of an image for the given [`Handle`](enum.Handle.md).

If the image is not already loaded, the [`Renderer`](trait.Renderer.md) may choose to return `None`, load the image in the background, and then trigger a relayout.

If you need a measurement right away, consider using [`Renderer::load_image`](trait.Renderer.md).

### `draw_image`

```rust
fn draw_image( &mut self, image: Image<Self::Handle>, bounds: Rectangle, clip_bounds: Rectangle, )
```

Draws an [`Image`](struct.Image.md) inside the provided `bounds`.

If the image is not already loaded, the [`Renderer`](trait.Renderer.md) may choose to render nothing, load the image in the background, and then trigger a redraw.

If you need to draw an image right away, consider using [`Renderer::load_image`](trait.Renderer.md) and hold on to an [`Allocation`](struct.Allocation.md) first.

## Implementors
