---
title: "raw_window_handle"
kind: module
path: "iced::window::raw_window_handle"
crate: "iced"
module: "iced::window"
file: "iced/window/raw_window_handle/index.md"
docs_url: "https://docs.iced.rs/iced/window/raw_window_handle/index.html"
---

# `iced::window::raw_window_handle`

Interoperability library for Rust Windowing applications.

This library provides standard types for accessing a window’s platform-specific raw window handle and platforms display handle. This does not provide any utilities for creating and managing windows; instead, it provides a common interface that window creation libraries (e.g. Winit, SDL) can use to easily talk with graphics libraries (e.g. gfx-hal).

### Safety guarantees

Please see the docs of [`HasWindowHandle`](trait.HasWindowHandle.md) and [`HasDisplayHandle`](trait.HasDisplayHandle.md).

### Platform handle initialization

Each platform handle struct is purposefully non-exhaustive, so that additional fields may be added without breaking backwards compatibility. Each struct provides an `empty` method that may be used along with the struct update syntax to construct it. See each specific struct for examples.

### Display Handles

Some windowing systems use a separate display handle for some operations. The display usually represents a connection to some display server, but it is not necessarily tied to a particular window. See [`RawDisplayHandle`](enum.RawDisplayHandle.md) for more details.

## Structs

- [`AndroidDisplayHandle`](struct.AndroidDisplayHandle.md) — Raw display handle for Android.
- [`AndroidNdkWindowHandle`](struct.AndroidNdkWindowHandle.md) — Raw window handle for Android NDK.
- [`AppKitDisplayHandle`](struct.AppKitDisplayHandle.md) — Raw display handle for AppKit.
- [`AppKitWindowHandle`](struct.AppKitWindowHandle.md) — Raw window handle for AppKit.
- [`DisplayHandle`](struct.DisplayHandle.md) — The handle to the display controller of the windowing system.
- [`DrmDisplayHandle`](struct.DrmDisplayHandle.md) — Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
- [`DrmWindowHandle`](struct.DrmWindowHandle.md) — Raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager.
- [`GbmDisplayHandle`](struct.GbmDisplayHandle.md) — Raw display handle for the Linux Generic Buffer Manager.
- [`GbmWindowHandle`](struct.GbmWindowHandle.md) — Raw window handle for the Linux Generic Buffer Manager.
- [`HaikuDisplayHandle`](struct.HaikuDisplayHandle.md) — Raw display handle for Haiku.
- [`HaikuWindowHandle`](struct.HaikuWindowHandle.md) — Raw window handle for Haiku.
- [`OhosDisplayHandle`](struct.OhosDisplayHandle.md) — Raw display handle for OpenHarmony.
- [`OhosNdkWindowHandle`](struct.OhosNdkWindowHandle.md) — Raw window handle for Ohos NDK.
- [`OrbitalDisplayHandle`](struct.OrbitalDisplayHandle.md) — Raw display handle for the Redox operating system.
- [`OrbitalWindowHandle`](struct.OrbitalWindowHandle.md) — Raw window handle for the Redox operating system.
- [`UiKitDisplayHandle`](struct.UiKitDisplayHandle.md) — Raw display handle for UIKit.
- [`UiKitWindowHandle`](struct.UiKitWindowHandle.md) — Raw window handle for UIKit.
- [`WaylandDisplayHandle`](struct.WaylandDisplayHandle.md) — Raw display handle for Wayland.
- [`WaylandWindowHandle`](struct.WaylandWindowHandle.md) — Raw window handle for Wayland.
- [`WebCanvasWindowHandle`](struct.WebCanvasWindowHandle.md) — Raw window handle for a Web canvas registered via [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).
- [`WebDisplayHandle`](struct.WebDisplayHandle.md) — Raw display handle for the Web.
- [`WebOffscreenCanvasWindowHandle`](struct.WebOffscreenCanvasWindowHandle.md) — Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).
- [`WebWindowHandle`](struct.WebWindowHandle.md) — Raw window handle for the Web.
- [`Win32WindowHandle`](struct.Win32WindowHandle.md) — Raw window handle for Win32.
- [`WinRtWindowHandle`](struct.WinRtWindowHandle.md) — Raw window handle for WinRT.
- [`WindowHandle`](struct.WindowHandle.md) — The handle to a window.
- [`WindowsDisplayHandle`](struct.WindowsDisplayHandle.md) — Raw display handle for Windows.
- [`XcbDisplayHandle`](struct.XcbDisplayHandle.md) — Raw display handle for Xcb.
- [`XcbWindowHandle`](struct.XcbWindowHandle.md) — Raw window handle for Xcb.
- [`XlibDisplayHandle`](struct.XlibDisplayHandle.md) — Raw display handle for Xlib.
- [`XlibWindowHandle`](struct.XlibWindowHandle.md) — Raw window handle for Xlib.

## Enums

- [`HandleError`](enum.HandleError.md) — An error that can occur while fetching a display or window handle.
- [`RawDisplayHandle`](enum.RawDisplayHandle.md) — A display server handle for a particular windowing system.
- [`RawWindowHandle`](enum.RawWindowHandle.md) — A window handle for a particular windowing system.

## Traits

- [`HasDisplayHandle`](trait.HasDisplayHandle.md) — A display that acts as a wrapper around a display handle.
- [`HasRawDisplayHandle`](trait.HasRawDisplayHandle.md) *(Deprecated)* — Display that wraps around a raw display handle.
- [`HasRawWindowHandle`](trait.HasRawWindowHandle.md) *(Deprecated)* — Window that wraps around a raw window handle.
- [`HasWindowHandle`](trait.HasWindowHandle.md) — A handle to a window.
