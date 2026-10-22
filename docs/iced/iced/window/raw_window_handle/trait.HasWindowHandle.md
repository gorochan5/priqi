---
title: "HasWindowHandle"
kind: trait
path: "iced::window::raw_window_handle::HasWindowHandle"
crate: "iced"
module: "iced::window::raw_window_handle"
file: "iced/window/raw_window_handle/trait.HasWindowHandle.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/trait.HasWindowHandle.html"
---

# `iced::window::raw_window_handle::HasWindowHandle`

```rust
pub trait HasWindowHandle {
    // Required method
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError>;
}
```

A handle to a window.

Objects that implement this trait should be able to return a [`WindowHandle`](struct.WindowHandle.md) for the window that they are associated with. This handle should last for the lifetime of the object, and should return an error if the application is inactive.

Implementors of this trait will be windowing systems, like [`winit`](https://crates.io/crates/winit) and [`sdl2`](https://crates.io/crates/sdl2). These windowing systems should implement this trait on types that represent windows.

Users of this trait will include graphics libraries, like [`wgpu`](https://crates.io/crates/wgpu) and [`glutin`](https://crates.io/crates/glutin). These APIs should be generic over a type that implements `HasWindowHandle`, and should use the [`WindowHandle`](struct.WindowHandle.md) type to access the window handle. The window handle should be acquired and held while the window is being used, in order to ensure that the window is not deleted while it is in use.

## Required Methods

### `window_handle`

```rust
fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError>
```

Get a handle to the window.

## Implementors

- `impl HasWindowHandle for Headless`
- `impl HasWindowHandle for WindowHandle<'_>`
