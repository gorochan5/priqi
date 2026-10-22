---
title: "scrollable"
kind: module
path: "iced::advanced::widget::operation::scrollable"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/scrollable/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/scrollable/index.html"
---

# `iced::advanced::widget::operation::scrollable`

Operate on widgets that can be scrolled.

## Structs

- [`AbsoluteOffset`](struct.AbsoluteOffset.md) — The amount of absolute offset in each direction of a [`Scrollable`](../trait.Scrollable.md).
- [`RelativeOffset`](struct.RelativeOffset.md) — The amount of relative offset in each direction of a [`Scrollable`](../trait.Scrollable.md).

## Traits

- [`Scrollable`](trait.Scrollable.md) — The internal state of a widget that can be scrolled.

## Functions

- [`scroll_by`](fn.scroll_by.md) — Produces an [`Operation`](../../trait.Operation.md) that scrolls the widget with the given [`Id`](../../../../widget/struct.Id.md) by the provided [`AbsoluteOffset`](../../../../widget/scrollable/struct.AbsoluteOffset.md).
- [`scroll_to`](fn.scroll_to.md) — Produces an [`Operation`](../../trait.Operation.md) that scrolls the widget with the given [`Id`](../../../../widget/struct.Id.md) to the provided [`AbsoluteOffset`](../../../../widget/scrollable/struct.AbsoluteOffset.md).
- [`snap_to`](fn.snap_to.md) — Produces an [`Operation`](../../trait.Operation.md) that snaps the widget with the given [`Id`](../../../../widget/struct.Id.md) to the provided `percentage`.
