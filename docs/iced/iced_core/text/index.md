---
title: "text"
kind: module
path: "iced_core::text"
crate: "iced_core"
module: "iced_core"
file: "iced_core/text/index.md"
docs_url: "https://docs.iced.rs/iced_core/text/index.html"
source_file: "src/iced_core/text.rs"
source_lines: "1-727"
---

# `iced_core::text`

Draw and interact with text.

## Re-exports

- [`Editor`](editor/trait.Editor.md)
- [`Highlighter`](highlighter/trait.Highlighter.md)
- [`Input`](input/struct.Input.md)
- [`Paragraph`](paragraph/trait.Paragraph.md)
- [`Parser`](parser/trait.Parser.md)

## Modules

- [`editor`](editor/index.md) — Edit text.
- [`highlighter`](highlighter/index.md) — Highlight text.
- [`input`](input/index.md)
- [`paragraph`](paragraph/index.md) — Draw paragraphs.
- [`parser`](parser/index.md) — Parse text.

## Structs

- [`Highlight`](struct.Highlight.md) — A text highlight.
- [`Hit`](struct.Hit.md) — The result of hit testing on text.
- [`Position`](struct.Position.md) — A specific position in some [`Text`](struct.Text.md).
- [`Span`](struct.Span.md) — A span of text.
- [`Text`](struct.Text.md) — A paragraph.

## Enums

- [`Alignment`](enum.Alignment.md) — The alignment of some text.
- [`Difference`](enum.Difference.md) — The difference detected in some text.
- [`Ellipsis`](enum.Ellipsis.md) — The ellipsis strategy of some text.
- [`LineHeight`](enum.LineHeight.md) — The height of a line of text in a paragraph.
- [`Shaping`](enum.Shaping.md) — The shaping strategy of some text.
- [`Target`](enum.Target.md) — The granularity of a text selection.
- [`Wrapping`](enum.Wrapping.md) — The wrapping strategy of some text.

## Traits

- [`IntoFragment`](trait.IntoFragment.md) — A trait for converting a value to some text [`Fragment`](type.Fragment.md).
- [`Renderer`](trait.Renderer.md) — A renderer capable of measuring and drawing [`Text`](struct.Text.md).

## Type Aliases

- [`Fragment`](type.Fragment.md) — A fragment of [`Text`](struct.Text.md).
