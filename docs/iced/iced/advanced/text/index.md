---
title: "text"
kind: module
path: "iced::advanced::text"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/text/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/text/index.html"
---

# `iced::advanced::text`

Draw and interact with text.

## Modules

- [`editor`](editor/index.md) — Edit text.
- [`highlighter`](highlighter/index.md) — Highlight text.
- [`input`](input/index.md)
- [`paragraph`](paragraph/index.md) — Draw paragraphs.
- [`parser`](parser/index.md) — Parse text.

## Structs

- [`Highlight`](struct.Highlight.md) — A text highlight.
- [`Hit`](struct.Hit.md) — The result of hit testing on text.
- [`Input`](struct.Input.md)
- [`Position`](struct.Position.md) — A specific position in some [`Text`](../struct.Text.md).
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

- [`Editor`](trait.Editor.md) — A component that can be used by widgets to edit multi-line text.
- [`Highlighter`](trait.Highlighter.md) — A type that describes how to highlight an `Input` with some [`Style`](highlighter/struct.Style.md).
- [`IntoFragment`](trait.IntoFragment.md) — A trait for converting a value to some text [`Fragment`](type.Fragment.md).
- [`Paragraph`](trait.Paragraph.md) — A text paragraph.
- [`Parser`](trait.Parser.md) — A type capable of parsing text, producing some `Output`.
- [`Renderer`](trait.Renderer.md) — A renderer capable of measuring and drawing [`Text`](../struct.Text.md).

## Type Aliases

- [`Fragment`](type.Fragment.md) — A fragment of [`Text`](../struct.Text.md).
