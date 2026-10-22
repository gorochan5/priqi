---
title: "Compositor"
kind: trait
path: "iced_graphics::compositor::Compositor"
crate: "iced_graphics"
module: "iced_graphics::compositor"
file: "iced_graphics/compositor/trait.Compositor.md"
docs_url: "https://docs.iced.rs/iced_graphics/compositor/trait.Compositor.html"
source_file: "src/iced_graphics/compositor.rs"
source_lines: "17-100"
---

# `iced_graphics::compositor::Compositor`

```rust
pub trait Compositor: Sized {
    type Renderer;
    type Surface;

    // Required methods
    fn new(
        settings: Settings,
        display: impl Display + Clone,
        compatible_window: impl Window + Clone,
        shell: Shell,
    ) -> impl Future<Output = Result<Self, Error>>;
    fn create_renderer(&self, settings: Settings) -> Self::Renderer;
    fn create_surface(
        &mut self,
        window: impl Window + Clone,
        width: u32,
        height: u32,
    ) -> Self::Surface;
    fn configure_surface(
        &mut self,
        surface: &mut Self::Surface,
        width: u32,
        height: u32,
    );
    fn information(&self) -> Information;
    fn present(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        on_pre_present: impl FnOnce(),
    ) -> Result<(), SurfaceError>;
    fn screenshot(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        background_color: Color,
    ) -> Vec<u8> ⓘ;

    // Provided methods
    fn load_font(&mut self, font: Cow<'static, [u8]>) -> Result<(), Error> { ... }
    fn list_fonts(&mut self) -> Result<Vec<Family>, Error> { ... }
}
```

A graphics compositor that can draw to windows.

**Source:** [compositor.rs](../../src/iced_graphics/compositor.rs.md) (lines 17-100)

## Required Associated Types

### `Renderer`

```rust
type Renderer
```

The iced renderer of the backend.

### `Surface`

```rust
type Surface
```

The surface of the backend.

## Required Methods

### `new`

```rust
fn new( settings: Settings, display: impl Display + Clone, compatible_window: impl Window + Clone, shell: Shell, ) -> impl Future<Output = Result<Self, Error>>
```

Creates a new [`Compositor`](trait.Compositor.md) with the given [`backend::Settings`].

### `create_renderer`

```rust
fn create_renderer(&self, settings: Settings) -> Self::Renderer
```

Creates a [`Self::Renderer`](trait.Compositor.md) for the [`Compositor`](trait.Compositor.md).

### `create_surface`

```rust
fn create_surface( &mut self, window: impl Window + Clone, width: u32, height: u32, ) -> Self::Surface
```

Crates a new [`Surface`](trait.Compositor.md) for the given window.

### `configure_surface`

```rust
fn configure_surface( &mut self, surface: &mut Self::Surface, width: u32, height: u32, )
```

Configures a new [`Surface`](trait.Compositor.md) with the given dimensions.

### `information`

```rust
fn information(&self) -> Information
```

Returns [`Information`](struct.Information.md) used by this [`Compositor`](trait.Compositor.md).

### `present`

```rust
fn present( &mut self, renderer: &mut Self::Renderer, surface: &mut Self::Surface, viewport: &Viewport, background_color: Color, on_pre_present: impl FnOnce(), ) -> Result<(), SurfaceError>
```

Presents the [`Renderer`](trait.Compositor.md) primitives to the next frame of the given [`Surface`](trait.Compositor.md).

### `screenshot`

```rust
fn screenshot( &mut self, renderer: &mut Self::Renderer, viewport: &Viewport, background_color: Color, ) -> Vec<u8> ⓘ
```

Screenshots the current [`Renderer`](trait.Compositor.md) primitives to an offscreen texture, and returns the bytes of the texture ordered as `RGBA` in the `sRGB` color space.

## Provided Methods

### `load_font`

```rust
fn load_font(&mut self, font: Cow<'static, [u8]>) -> Result<(), Error>
```

Loads a font from its bytes.

### `list_fonts`

```rust
fn list_fonts(&mut self) -> Result<Vec<Family>, Error>
```

Lists all the available font families.

## Implementors
