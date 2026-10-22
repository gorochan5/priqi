---
title: "HasRawWindowHandle"
kind: trait
path: "iced::window::raw_window_handle::HasRawWindowHandle"
crate: "iced"
module: "iced::window::raw_window_handle"
file: "iced/window/raw_window_handle/trait.HasRawWindowHandle.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/trait.HasRawWindowHandle.html"
---

# `iced::window::raw_window_handle::HasRawWindowHandle`

```rust
pub unsafe trait HasRawWindowHandle {
    // Required method
    fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError>;
}
```

Window that wraps around a raw window handle.

## Safety

Users can safely assume that pointers and non-zero fields are valid, and it is up to the implementer of this trait to ensure that condition is upheld.

Despite that qualification, implementers should still make a best-effort attempt to fill in all available fields. If an implementation doesn’t, and a downstream user needs the field, it should try to derive the field from other fields the implementer *does* provide via whatever methods the platform provides.

The exact handles returned by `raw_window_handle` must remain consistent between multiple calls to `raw_window_handle` as long as not indicated otherwise by platform specific events.

## Required Methods

### `raw_window_handle`

```rust
fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError>
```

*👎Deprecated: Use HasWindowHandle instead*

## Implementors

- `impl<T> HasRawWindowHandle for T where T: HasWindowHandle + ?Sized,`
