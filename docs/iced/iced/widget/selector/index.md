---
title: "selector"
kind: module
path: "iced::widget::selector"
crate: "iced"
module: "iced::widget"
file: "iced/widget/selector/index.md"
docs_url: "https://docs.iced.rs/iced/widget/selector/index.html"
---

# `iced::widget::selector`

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
- [`id`](fn.id.md) — Creates a new [`Selector`](trait.Selector.md) that matches widgets with the given [`widget::Id`](../struct.Id.md).
- [`is_focused`](fn.is_focused.md) — Returns a [`Selector`](trait.Selector.md) that matches widgets that are currently focused.
