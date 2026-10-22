---
title: "scrollable"
kind: module
path: "iced::widget::operation::scrollable"
crate: "iced"
module: "iced::widget::operation"
file: "iced/widget/operation/scrollable/index.md"
docs_url: "https://docs.iced.rs/iced/widget/operation/scrollable/index.html"
---

# `iced::widget::operation::scrollable`

Operations for widgets that can scroll.

## Structs

- [`AbsoluteOffset`](struct.AbsoluteOffset.md) — The amount of absolute offset in each direction of a [`Scrollable`](../../../advanced/widget/operation/trait.Scrollable.md).
- [`RelativeOffset`](struct.RelativeOffset.md) — The amount of relative offset in each direction of a [`Scrollable`](../../../advanced/widget/operation/trait.Scrollable.md).

## Functions

- [`scroll_by`](fn.scroll_by.md) — Scrolls the scrollable with the given [`Id`](../../struct.Id.md) by the provided [`AbsoluteOffset`](../../scrollable/struct.AbsoluteOffset.md).
- [`scroll_to`](fn.scroll_to.md) — Scrolls the scrollable with the given [`Id`](../../struct.Id.md) to the provided [`AbsoluteOffset`](../../scrollable/struct.AbsoluteOffset.md).
- [`snap_to`](fn.snap_to.md) — Snaps the scrollable with the given [`Id`](../../struct.Id.md) to the provided [`RelativeOffset`](../../scrollable/struct.RelativeOffset.md).
- [`snap_to_end`](fn.snap_to_end.md) — Snaps the scrollable with the given [`Id`](../../struct.Id.md) to the [`RelativeOffset::END`](../../scrollable/struct.RelativeOffset.md).
