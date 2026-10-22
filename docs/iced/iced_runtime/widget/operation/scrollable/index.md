---
title: "scrollable"
kind: module
path: "iced_runtime::widget::operation::scrollable"
crate: "iced_runtime"
module: "iced_runtime::widget::operation"
file: "iced_runtime/widget/operation/scrollable/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/scrollable/index.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "30"
---

# `iced_runtime::widget::operation::scrollable`

Operations for widgets that can scroll.

## Structs

- [`AbsoluteOffset`](struct.AbsoluteOffset.md) — The amount of absolute offset in each direction of a [`Scrollable`].
- [`RelativeOffset`](struct.RelativeOffset.md) — The amount of relative offset in each direction of a [`Scrollable`].

## Functions

- [`scroll_by`](fn.scroll_by.md) — Scrolls the scrollable with the given [`Id`] by the provided [`AbsoluteOffset`](struct.AbsoluteOffset.md).
- [`scroll_to`](fn.scroll_to.md) — Scrolls the scrollable with the given [`Id`] to the provided [`AbsoluteOffset`](struct.AbsoluteOffset.md).
- [`snap_to`](fn.snap_to.md) — Snaps the scrollable with the given [`Id`] to the provided [`RelativeOffset`](struct.RelativeOffset.md).
- [`snap_to_end`](fn.snap_to_end.md) — Snaps the scrollable with the given [`Id`] to the [`RelativeOffset::END`](struct.RelativeOffset.md).
