---
title: "operation"
kind: module
path: "iced::advanced::widget::operation"
crate: "iced"
module: "iced::advanced::widget"
file: "iced/advanced/widget/operation/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/index.html"
---

# `iced::advanced::widget::operation`

Query or update internal widget state.

## Modules

- [`focusable`](focusable/index.md) — Operate on widgets that can be focused.
- [`scrollable`](scrollable/index.md) — Operate on widgets that can be scrolled.
- [`text`](text/index.md) — Operate on widgets that have text.
- [`text_input`](text_input/index.md) — Operate on widgets that have text input.

## Enums

- [`Animation`](enum.Animation.md) — The animation to apply to a state change, as requested by an [`Operation`](../trait.Operation.md).
- [`Outcome`](enum.Outcome.md) — The result of an [`Operation`](../trait.Operation.md).

## Traits

- [`Focusable`](trait.Focusable.md) — The internal state of a widget that can be focused.
- [`Operation`](trait.Operation.md) — A piece of logic that can traverse the widget tree of an application in order to query or update some widget state.
- [`Scrollable`](trait.Scrollable.md) — The internal state of a widget that can be scrolled.
- [`Text`](trait.Text.md) — The internal state of a widget that has text.
- [`TextInput`](trait.TextInput.md) — The internal state of a widget that has text input.

## Functions

- [`black_box`](fn.black_box.md) — Wraps the [`Operation`](../trait.Operation.md) in a black box, erasing its returning type.
- [`map`](fn.map.md) — Maps the output of an [`Operation`](../trait.Operation.md) using the given function.
- [`scope`](fn.scope.md) — Produces an [`Operation`](../trait.Operation.md) that applies the given [`Operation`](../trait.Operation.md) to the children of a container with the given [`Id`](../../../widget/struct.Id.md).
- [`then`](fn.then.md) — Chains the output of an [`Operation`](../trait.Operation.md) with the provided function to build a new [`Operation`](../trait.Operation.md).
