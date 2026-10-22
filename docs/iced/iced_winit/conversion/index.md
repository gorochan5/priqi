---
title: "conversion"
kind: module
path: "iced_winit::conversion"
crate: "iced_winit"
module: "iced_winit"
file: "iced_winit/conversion/index.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/index.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "1-1149"
---

# `iced_winit::conversion`

Convert [`winit`](https://github.com/rust-windowing/winit) types into [`iced_runtime`](https://github.com/iced-rs/iced/tree/master/runtime) types, and viceversa.

## Functions

- [`cursor_position`](fn.cursor_position.md) — Converts a physical cursor position into a logical `Point`.
- [`fullscreen`](fn.fullscreen.md) — Converts a [`window::Mode`] into a [`winit`](https://github.com/rust-windowing/winit) fullscreen mode.
- [`icon`](fn.icon.md) — Converts some [`window::Icon`] into its `winit` counterpart.
- [`ime_purpose`](fn.ime_purpose.md) — Converts some [`input_method::Purpose`] into its `winit` counterpart.
- [`key`](fn.key.md) — Converts a `Key` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) key.
- [`key_code`](fn.key_code.md) — Converts a `KeyCode` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) key code.
- [`mode`](fn.mode.md) — Converts a [`winit`](https://github.com/rust-windowing/winit) fullscreen mode into a [`window::Mode`].
- [`modifiers`](fn.modifiers.md) — Converts some `ModifiersState` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) modifiers state.
- [`mouse_button`](fn.mouse_button.md) — Converts a `MouseButton` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) mouse button.
- [`mouse_interaction`](fn.mouse_interaction.md) — Converts a [`mouse::Interaction`] into a [`winit`](https://github.com/rust-windowing/winit) cursor icon.
- [`native_key_code`](fn.native_key_code.md) — Converts a `NativeKeyCode` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) native key code.
- [`physical_key`](fn.physical_key.md) — Converts a `PhysicalKey` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) physical key.
- [`position`](fn.position.md) — Converts a [`window::Position`] into a [`winit`](https://github.com/rust-windowing/winit) logical position for a given monitor.
- [`resize_direction`](fn.resize_direction.md) — Converts some [`window::Direction`] into a [`winit::window::ResizeDirection`].
- [`theme_mode`](fn.theme_mode.md) — Converts a [`winit`](https://github.com/rust-windowing/winit) window theme into a [`theme::Mode`].
- [`touch_event`](fn.touch_event.md) — Converts a `Touch` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) touch event.
- [`user_attention`](fn.user_attention.md) — Converts some `UserAttention` into its `winit` counterpart.
- [`visible`](fn.visible.md) — Converts a [`window::Mode`] into a visibility flag.
- [`window_attributes`](fn.window_attributes.md) — Converts some [`window::Settings`] into some `WindowAttributes` from `winit`.
- [`window_event`](fn.window_event.md) — Converts a winit window event into an iced event.
- [`window_level`](fn.window_level.md) — Converts a [`window::Level`] into a [`winit`](https://github.com/rust-windowing/winit) window level.
- [`window_theme`](fn.window_theme.md) — Converts a [`theme::Mode`] into a window theme.
