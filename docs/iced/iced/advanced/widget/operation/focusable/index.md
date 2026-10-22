---
title: "focusable"
kind: module
path: "iced::advanced::widget::operation::focusable"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/focusable/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/focusable/index.html"
---

# `iced::advanced::widget::operation::focusable`

Operate on widgets that can be focused.

## Structs

- [`Count`](struct.Count.md) — A summary of the focusable widgets present on a widget tree.

## Traits

- [`Focusable`](trait.Focusable.md) — The internal state of a widget that can be focused.

## Functions

- [`count`](fn.count.md) — Produces an [`Operation`](../../trait.Operation.md) that generates a [`Count`](struct.Count.md) and chains it with the provided function to build a new [`Operation`](../../trait.Operation.md).
- [`find_focused`](fn.find_focused.md) — Produces an [`Operation`](../../trait.Operation.md) that searches for the current focused widget and stores its ID. This ignores widgets that do not have an ID.
- [`focus`](fn.focus.md) — Produces an [`Operation`](../../trait.Operation.md) that focuses the widget with the given [`Id`](../../../../widget/struct.Id.md).
- [`focus_next`](fn.focus_next.md) — Produces an [`Operation`](../../trait.Operation.md) that searches for the current focused widget, and
- [`focus_previous`](fn.focus_previous.md) — Produces an [`Operation`](../../trait.Operation.md) that searches for the current focused widget, and
- [`is_focused`](fn.is_focused.md) — Produces an [`Operation`](../../trait.Operation.md) that searches for the focusable widget and stores whether it is focused or not. This ignores widgets that do not have an ID.
- [`unfocus`](fn.unfocus.md) — Produces an [`Operation`](../../trait.Operation.md) that unfocuses the focused widget.
