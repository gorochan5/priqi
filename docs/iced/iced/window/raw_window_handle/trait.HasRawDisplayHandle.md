---
title: "HasRawDisplayHandle"
kind: trait
path: "iced::window::raw_window_handle::HasRawDisplayHandle"
crate: "iced"
module: "iced::window::raw_window_handle"
file: "iced/window/raw_window_handle/trait.HasRawDisplayHandle.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/trait.HasRawDisplayHandle.html"
---

# `iced::window::raw_window_handle::HasRawDisplayHandle`

```rust
pub unsafe trait HasRawDisplayHandle {
    // Required method
    fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError>;
}
```

Display that wraps around a raw display handle.

## Safety

Users can safely assume that pointers and non-zero fields are valid, and it is up to the implementer of this trait to ensure that condition is upheld.

Despite that qualification, implementers should still make a best-effort attempt to fill in all available fields. If an implementation doesn’t, and a downstream user needs the field, it should try to derive the field from other fields the implementer *does* provide via whatever methods the platform provides.

The exact handles returned by `raw_display_handle` must remain consistent between multiple calls to `raw_display_handle` as long as not indicated otherwise by platform specific events.

## Required Methods

### `raw_display_handle`

```rust
fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError>
```

*👎Deprecated: Use HasDisplayHandle instead*

## Implementors

- `impl<T> HasRawDisplayHandle for T where T: HasDisplayHandle + ?Sized,`
