---
title: "tree"
kind: module
path: "iced_core::widget::tree"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/tree/index.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/index.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "1-284"
---

# `iced_core::widget::tree`

Store internal widget state in a state tree to ensure continuity.

## Structs

- [`Tag`](struct.Tag.md) — The identifier of some widget state.
- [`Tree`](struct.Tree.md) — A persistent state widget tree.

## Enums

- [`State`](enum.State.md) — The internal [`State`](enum.State.md) of a widget.

## Functions

- [`diff_children`](fn.diff_children.md) — Reconciles the children of the tree with the provided list of widgets.
- [`diff_children_custom`](fn.diff_children_custom.md) — Reconciles the children of the tree with the provided list of widgets using custom logic both for diffing and creating new widget state.
- [`diff_children_custom_with_search`](fn.diff_children_custom_with_search.md) — Reconciles the `current_children` with the provided list of widgets using custom logic both for diffing and creating new widget state.
