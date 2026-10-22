---
title: "compositor"
kind: module
path: "iced_wgpu::window::compositor"
crate: "iced_wgpu"
module: "iced_wgpu::window"
file: "iced_wgpu/window/compositor/index.md"
docs_url: "https://docs.iced.rs/iced_wgpu/window/compositor/index.html"
source_file: "src/iced_wgpu/window/compositor.rs"
source_lines: "1-458"
---

# `iced_wgpu::window::compositor`

Connect a window with a renderer.

## Structs

- [`Compositor`](struct.Compositor.md) — A window graphics backend for iced powered by `wgpu`.
- [`Settings`](struct.Settings.md) — The settings of a [`Compositor`](struct.Compositor.md).

## Enums

- [`Error`](enum.Error.md) — A compositor error.

## Functions

- [`new`](fn.new.md) — Creates a [`Compositor`](struct.Compositor.md) with the given [`Settings`](struct.Settings.md) and window.
- [`present`](fn.present.md) — Presents the given primitives with the given [`Compositor`](struct.Compositor.md).
- [`present_mode_from_env`](fn.present_mode_from_env.md) — Obtains a [`wgpu::PresentMode`] from the current environment configuration, if set.
