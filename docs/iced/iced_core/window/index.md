---
title: "window"
kind: module
path: "iced_core::window"
crate: "iced_core"
module: "iced_core"
file: "iced_core/window/index.md"
docs_url: "https://docs.iced.rs/iced_core/window/index.html"
source_file: "src/iced_core/window.rs"
source_lines: "1-55"
---

# `iced_core::window`

Build window-based GUI applications.

## Re-exports

- [`Icon`](icon/struct.Icon.md)
- [`Screenshot`](screenshot/struct.Screenshot.md)
- [`Settings`](settings/struct.Settings.md)
- `pub use raw_window_handle;`

## Modules

- [`icon`](icon/index.md) — Change the icon of a window.
- [`screenshot`](screenshot/index.md) — Take screenshots of a window.
- [`settings`](settings/index.md) — Configure your windows.

## Structs

- [`Headless`](struct.Headless.md) — A headless window.
- [`Id`](struct.Id.md) — The id of the window.

## Enums

- [`Direction`](enum.Direction.md) — The cardinal directions relative to the center of a window.
- [`Event`](enum.Event.md) — A window-related event.
- [`Level`](enum.Level.md) — A window level groups windows with respect to their z-position.
- [`Mode`](enum.Mode.md) — The mode of a window-based application.
- [`Position`](enum.Position.md) — The position of a window in a given screen.
- [`RedrawRequest`](enum.RedrawRequest.md) — A request to redraw a window.
- [`UserAttention`](enum.UserAttention.md) — The type of user attention to request.

## Traits

- [`Window`](trait.Window.md) — A window managed by iced.
