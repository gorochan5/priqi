---
title: "HasDisplayHandle"
kind: trait
path: "iced::window::raw_window_handle::HasDisplayHandle"
crate: "iced"
module: "iced::window::raw_window_handle"
file: "iced/window/raw_window_handle/trait.HasDisplayHandle.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/trait.HasDisplayHandle.html"
---

# `iced::window::raw_window_handle::HasDisplayHandle`

```rust
pub trait HasDisplayHandle {
    // Required method
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}
```

A display that acts as a wrapper around a display handle.

Objects that implement this trait should be able to return a [`DisplayHandle`](struct.DisplayHandle.md) for the display that they are associated with. This handle should last for the lifetime of the object, and should return an error if the application is inactive.

Implementors of this trait will be windowing systems, like [`winit`](https://crates.io/crates/winit) and [`sdl2`](https://crates.io/crates/sdl2). These windowing systems should implement this trait on types that represent the top-level display server. It should be implemented by tying the lifetime of the [`DisplayHandle`](struct.DisplayHandle.md) to the lifetime of the display object.

Users of this trait will include graphics libraries, like [`wgpu`](https://crates.io/crates/wgpu) and [`glutin`](https://crates.io/crates/glutin). These APIs should be generic over a type that implements `HasDisplayHandle`, and should use the [`DisplayHandle`](struct.DisplayHandle.md) type to access the display handle.

Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the constructors of [`DisplayHandle`](struct.DisplayHandle.md). This is because the `HasDisplayHandle` trait is safe to implement.

## Required Methods

### `display_handle`

```rust
fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>
```

Get a handle to the display controller of the windowing system.

## Implementors

- `impl HasDisplayHandle for Headless`
- `impl<'a> HasDisplayHandle for DisplayHandle<'a>`
