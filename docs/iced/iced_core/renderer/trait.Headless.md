---
title: "Headless"
kind: trait
path: "iced_core::renderer::Headless"
crate: "iced_core"
module: "iced_core::renderer"
file: "iced_core/renderer/trait.Headless.md"
docs_url: "https://docs.iced.rs/iced_core/renderer/trait.Headless.html"
source_file: "src/iced_core/renderer.rs"
source_lines: "143-163"
---

# `iced_core::renderer::Headless`

```rust
pub trait Headless {
    // Required methods
    fn new(
        settings: Settings,
        backend: Option<&str>,
    ) -> impl Future<Output = Option<Self>>
       where Self: Sized;
    fn name(&self) -> String;
    fn screenshot(
        &mut self,
        size: Size<u32>,
        scale_factor: f32,
        background_color: Color,
    ) -> Vec<u8> ⓘ;
}
```

A headless renderer is a renderer that can render offscreen without a window nor a compositor.

**Source:** [renderer.rs](../../src/iced_core/renderer.rs.md) (lines 143-163)

## Required Methods

### `new`

```rust
fn new( settings: Settings, backend: Option<&str>, ) -> impl Future<Output = Option<Self>> where Self: Sized,
```

Creates a new [`Headless`](trait.Headless.md) renderer;

### `name`

```rust
fn name(&self) -> String
```

Returns the unique name of the renderer.

This name may be used by testing libraries to uniquely identify snapshots.

### `screenshot`

```rust
fn screenshot( &mut self, size: Size<u32>, scale_factor: f32, background_color: Color, ) -> Vec<u8> ⓘ
```

Draws offscreen into a screenshot, returning a collection of bytes representing the rendered pixels in RGBA order.

## Implementors
