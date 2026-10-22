---
title: "selector"
kind: module
path: "iced_runtime::widget::selector"
crate: "iced_runtime"
module: "iced_runtime::widget"
file: "iced_runtime/widget/selector/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/selector/index.html"
source_file: "src/iced_runtime/widget/selector.rs"
source_lines: "1-23"
---

# `iced_runtime::widget::selector`

Find and query widgets in your applications.

## Enums

- [`Candidate`](enum.Candidate.md) — A selection candidate.
- [`Target`](enum.Target.md) — A generic widget match produced during selection.
- [`Text`](enum.Text.md) — A text match.

## Traits

- [`Bounded`](trait.Bounded.md) — A bounded type has both layout bounds and visible bounds.
- [`Selector`](trait.Selector.md) — A type that traverses the widget tree to “select” data and produce some output.

## Functions

- [`find`](fn.find.md) — Finds a widget matching the given [`Selector`](trait.Selector.md).
- [`find_all`](fn.find_all.md) — Finds all widgets matching the given [`Selector`](trait.Selector.md).
- [`id`](fn.id.md) — Creates a new [`Selector`](trait.Selector.md) that matches widgets with the given [`widget::Id`].
- [`is_focused`](fn.is_focused.md) — Returns a [`Selector`](trait.Selector.md) that matches widgets that are currently focused.
