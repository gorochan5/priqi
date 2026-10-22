---
title: "widget"
kind: module
path: "iced_core::widget"
crate: "iced_core"
module: "iced_core"
file: "iced_core/widget/index.md"
docs_url: "https://docs.iced.rs/iced_core/widget/index.html"
source_file: "src/iced_core/widget.rs"
source_lines: "1-561"
---

# `iced_core::widget`

Create custom widgets and operate on them.

## Re-exports

- [`Operation`](operation/trait.Operation.md)
- [`Text`](text/struct.Text.md)
- [`Tree`](tree/struct.Tree.md)

## Modules

- [`operation`](operation/index.md) — Query or update internal widget state.
- [`text`](text/index.md) — Text widgets display information through writing.
- [`tree`](tree/index.md) — Store internal widget state in a state tree to ensure continuity.

## Structs

- [`Element`](struct.Element.md) — A generic [`Widget`](trait.Widget.md).
- [`Id`](struct.Id.md) — The identifier of a generic widget.
- [`Map`](struct.Map.md) — A [`Widget`](trait.Widget.md) that transforms the produced message of another widget.
- [`Void`](struct.Void.md) — A zero-sized [`Widget`](trait.Widget.md) that does nothing and will be filtered out by containers.

## Traits

- [`Meta`](trait.Meta.md) — Metadata of a [`Widget`](trait.Widget.md), independent of its generic parameters.
- [`Widget`](trait.Widget.md) — A component that displays information and allows interaction.
