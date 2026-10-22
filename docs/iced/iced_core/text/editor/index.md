---
title: "editor"
kind: module
path: "iced_core::text::editor"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/editor/index.md"
docs_url: "https://docs.iced.rs/iced_core/text/editor/index.html"
source_file: "src/iced_core/text/editor.rs"
source_lines: "1-1034"
---

# `iced_core::text::editor`

Edit text.

## Structs

- [`Cursor`](struct.Cursor.md) — The range of an [`Editor`](trait.Editor.md).
- [`KeyPress`](struct.KeyPress.md) — A key press.
- [`Line`](struct.Line.md) — A line of an [`Editor`](trait.Editor.md).
- [`State`](struct.State.md) — The internal state of an [`Editor`](trait.Editor.md).
- [`Style`](struct.Style.md) — The visual style of an [`Editor`](trait.Editor.md).

## Enums

- [`Action`](enum.Action.md) — An interaction with an [`Editor`](trait.Editor.md).
- [`Binding`](enum.Binding.md) — A binding to an action in the [`Editor`](trait.Editor.md).
- [`Direction`](enum.Direction.md) — A direction in some text.
- [`Edit`](enum.Edit.md) — An action that edits text.
- [`LineEnding`](enum.LineEnding.md) — The line ending of a [`Line`](struct.Line.md).
- [`Motion`](enum.Motion.md) — A cursor movement.
- [`Selection`](enum.Selection.md) — The cursor of an [`Editor`](trait.Editor.md).
- [`Update`](enum.Update.md) — The update of an [`Editor`](trait.Editor.md), returned by [`State::update`](struct.State.md).

## Traits

- [`Editor`](trait.Editor.md) — A component that can be used by widgets to edit multi-line text.
