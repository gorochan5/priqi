---
title: "widget"
kind: module
path: "iced::advanced::widget"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/widget/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/index.html"
source_file: "src/iced/advanced.rs"
source_lines: "9"
---

# `iced::advanced::widget`

Create custom widgets and operate on them.

## Modules

- [`operation`](operation/index.md) — Query or update internal widget state.
- [`text`](text/index.md) — Text widgets display information through writing.
- [`tree`](tree/index.md) — Store internal widget state in a state tree to ensure continuity.

## Structs

- [`Element`](struct.Element.md) — A generic [`Widget`](../trait.Widget.md).
- [`Id`](struct.Id.md) — The identifier of a generic widget.
- [`Map`](struct.Map.md) — A [`Widget`](../trait.Widget.md) that transforms the produced message of another widget.
- [`Text`](struct.Text.md) — A bunch of text.
- [`Tree`](struct.Tree.md) — A persistent state widget tree.
- [`Void`](struct.Void.md) — A zero-sized [`Widget`](../trait.Widget.md) that does nothing and will be filtered out by containers.

## Traits

- [`Meta`](trait.Meta.md) — Metadata of a [`Widget`](../trait.Widget.md), independent of its generic parameters.
- [`Operation`](trait.Operation.md) — A piece of logic that can traverse the widget tree of an application in order to query or update some widget state.
- [`Widget`](trait.Widget.md) — A component that displays information and allows interaction.

## Functions

- [`operate`](fn.operate.md) — Creates a new [`Task`](../../struct.Task.md) that runs the given [`widget::Operation`](trait.Operation.md) and produces its output.
