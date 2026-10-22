---
title: "operation"
kind: module
path: "iced_core::widget::operation"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/operation/index.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/index.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "1-462"
---

# `iced_core::widget::operation`

Query or update internal widget state.

## Re-exports

- [`Focusable`](focusable/trait.Focusable.md)
- [`Scrollable`](scrollable/trait.Scrollable.md)
- [`Text`](text/trait.Text.md)
- [`TextInput`](text_input/trait.TextInput.md)

## Modules

- [`focusable`](focusable/index.md) — Operate on widgets that can be focused.
- [`scrollable`](scrollable/index.md) — Operate on widgets that can be scrolled.
- [`text`](text/index.md) — Operate on widgets that have text.
- [`text_input`](text_input/index.md) — Operate on widgets that have text input.

## Enums

- [`Animation`](enum.Animation.md) — The animation to apply to a state change, as requested by an [`Operation`](trait.Operation.md).
- [`Outcome`](enum.Outcome.md) — The result of an [`Operation`](trait.Operation.md).

## Traits

- [`Operation`](trait.Operation.md) — A piece of logic that can traverse the widget tree of an application in order to query or update some widget state.

## Functions

- [`black_box`](fn.black_box.md) — Wraps the [`Operation`](trait.Operation.md) in a black box, erasing its returning type.
- [`map`](fn.map.md) — Maps the output of an [`Operation`](trait.Operation.md) using the given function.
- [`scope`](fn.scope.md) — Produces an [`Operation`](trait.Operation.md) that applies the given [`Operation`](trait.Operation.md) to the children of a container with the given [`Id`](../struct.Id.md).
- [`then`](fn.then.md) — Chains the output of an [`Operation`](trait.Operation.md) with the provided function to build a new [`Operation`](trait.Operation.md).
