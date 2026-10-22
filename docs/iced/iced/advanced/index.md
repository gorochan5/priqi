---
title: "advanced"
kind: module
path: "iced::advanced"
crate: "iced"
module: "iced"
file: "iced/advanced/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/index.html"
source_file: "src/iced/advanced.rs"
source_lines: "1-27"
---

# `iced::advanced`

Leverage advanced concepts like custom widgets.

## Re-exports

- `pub use crate::renderer::graphics;`

## Modules

- [`clipboard`](clipboard/index.md) — Access the clipboard.
- [`image`](image/index.md) — Load and draw raster graphics.
- [`input_method`](input_method/index.md) — Listen to input method events.
- [`layout`](layout/index.md) — Position your widgets properly.
- [`mouse`](mouse/index.md) — Handle mouse events.
- [`overlay`](overlay/index.md) — Display interactive elements on top of other widgets.
- [`renderer`](renderer/index.md) — Write your own renderer.
- [`shell`](shell/index.md) — Communicate with the iced runtime from widgets.
- [`subscription`](subscription/index.md) — Write your own subscriptions.
- [`svg`](svg/index.md) — Load and draw vector graphics.
- [`text`](text/index.md) — Draw and interact with text.
- [`widget`](widget/index.md) — Create custom widgets and operate on them.

## Structs

- [`Layout`](struct.Layout.md) — The bounds of a widget and its children, using absolute coordinates.
- [`Shell`](struct.Shell.md) — A connection to the state of a shell.
- [`Text`](struct.Text.md) — A paragraph.

## Enums

- [`InputMethod`](enum.InputMethod.md) — The input method strategy of a widget.

## Traits

- [`Overlay`](trait.Overlay.md) — An interactive component that can be displayed on top of other widgets.
- [`Renderer`](trait.Renderer.md) — A component that can be used by widgets to draw themselves on a screen.
- [`Widget`](trait.Widget.md) — A component that displays information and allows interaction.
