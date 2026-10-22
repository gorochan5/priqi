---
title: "text_input"
kind: module
path: "iced::advanced::widget::operation::text_input"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/text_input/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/text_input/index.html"
---

# `iced::advanced::widget::operation::text_input`

Operate on widgets that have text input.

## Traits

- [`TextInput`](trait.TextInput.md) — The internal state of a widget that has text input.

## Functions

- [`move_cursor_to`](fn.move_cursor_to.md) — Produces an [`Operation`](../../trait.Operation.md) that moves the cursor of the widget with the given [`Id`](../../../../widget/struct.Id.md) to the provided position.
- [`move_cursor_to_end`](fn.move_cursor_to_end.md) — Produces an [`Operation`](../../trait.Operation.md) that moves the cursor of the widget with the given [`Id`](../../../../widget/struct.Id.md) to the end.
- [`move_cursor_to_front`](fn.move_cursor_to_front.md) — Produces an [`Operation`](../../trait.Operation.md) that moves the cursor of the widget with the given [`Id`](../../../../widget/struct.Id.md) to the front.
- [`select_all`](fn.select_all.md) — Produces an [`Operation`](../../trait.Operation.md) that selects all the content of the widget with the given [`Id`](../../../../widget/struct.Id.md).
- [`select_range`](fn.select_range.md) — Produces an [`Operation`](../../trait.Operation.md) that selects the given content range of the widget with the given [`Id`](../../../../widget/struct.Id.md).
