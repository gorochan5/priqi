---
title: "layout"
kind: module
path: "iced::advanced::layout"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/layout/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/layout/index.html"
---

# `iced::advanced::layout`

Position your widgets properly.

## Modules

- [`flex`](flex/index.md) — Distribute elements using a flex-based layout.

## Structs

- [`Layout`](struct.Layout.md) — The bounds of a widget and its children, using absolute coordinates.
- [`Limits`](struct.Limits.md) — A set of size constraints for layouting.

## Functions

- [`atomic`](fn.atomic.md) — Computes the resulting [`Size`](../../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and no intrinsic size.
- [`contained`](fn.contained.md) — Computes the resulting [`Size`](../../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and a closure that produces the content inside the given [`Limits`](struct.Limits.md).
- [`next_to_each_other`](fn.next_to_each_other.md) — Lays out two children side by side in the provided [`widget::Tree`](../widget/struct.Tree.md).
- [`padded`](fn.padded.md) — Computes the [`Size`](../../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width, height, and [`Padding`](../../struct.Padding.md) requirements and a closure that produces the content inside the given [`Limits`](struct.Limits.md).
- [`positioned`](fn.positioned.md) — Computes a [`padded`](fn.padded.md) [`Size`](../../struct.Size.md) with a positioning step.
- [`sized`](fn.sized.md) — Computes the resulting [`Size`](../../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and a closure that produces the intrinsic [`Size`](../../struct.Size.md) inside the given [`Limits`](struct.Limits.md).
