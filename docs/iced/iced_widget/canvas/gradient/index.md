---
title: "gradient"
kind: module
path: "iced_widget::canvas::gradient"
crate: "iced_widget"
module: "iced_widget::canvas"
file: "iced_widget/canvas/gradient/index.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/gradient/index.html"
---

# `iced_widget::canvas::gradient`

A gradient that can be used as a fill for some geometry.

For a gradient that you can use as a background variant for a widget, see [`Gradient`](../enum.Gradient.md).

## Structs

- [`Linear`](struct.Linear.md) — A linear gradient.
- [`Packed`](struct.Packed.md) — Packed [`Gradient`](../enum.Gradient.md) data for use in shader code.

## Enums

- [`Gradient`](enum.Gradient.md) — A fill which linearly interpolates colors along a direction.

## Functions

- [`pack`](fn.pack.md) — Creates a new [`Packed`](struct.Packed.md) gradient for use in shader code.
