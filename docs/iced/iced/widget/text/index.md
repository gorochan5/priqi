---
title: "text"
kind: module
path: "iced::widget::text"
crate: "iced"
module: "iced::widget"
file: "iced/widget/text/index.md"
docs_url: "https://docs.iced.rs/iced/widget/text/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "43"
---

# `iced::widget::text`

Draw and interact with text.

## Modules

- [`highlighter`](highlighter/index.md) — Highlight text.

## Structs

- [`Format`](struct.Format.md) — The format of some [`Text`](../../advanced/widget/struct.Text.md).
- [`Operand`](struct.Operand.md) — The state of a widget with text, operated on by [`super::Operation::text`](../../advanced/widget/trait.Operation.md).
- [`Position`](struct.Position.md) — A specific position in some [`Text`](../../advanced/struct.Text.md).
- [`Rich`](struct.Rich.md) — A bunch of [`Rich`](struct.Rich.md) text.
- [`Span`](struct.Span.md) — A span of text.
- [`Style`](struct.Style.md) — The appearance of some text.

## Enums

- [`Alignment`](enum.Alignment.md) — The alignment of some text.
- [`Ellipsis`](enum.Ellipsis.md) — The ellipsis strategy of some text.
- [`LineHeight`](enum.LineHeight.md) — The height of a line of text in a paragraph.
- [`Shaping`](enum.Shaping.md) — The shaping strategy of some text.
- [`Wrapping`](enum.Wrapping.md) — The wrapping strategy of some text.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Text`](../../advanced/widget/struct.Text.md).
- [`Highlighter`](trait.Highlighter.md) — A type that describes how to highlight an `Input` with some [`Style`](../../advanced/text/highlighter/struct.Style.md).
- [`IntoFragment`](trait.IntoFragment.md) — A trait for converting a value to some text [`Fragment`](../../advanced/text/type.Fragment.md).
- [`Parser`](trait.Parser.md) — A type capable of parsing text, producing some `Output`.

## Functions

- [`base`](fn.base.md) — Text with the default base color.
- [`danger`](fn.danger.md) — Text conveying some negative information, like an error.
- [`default`](fn.default.md) — The default text styling; color is inherited.
- [`draw`](fn.draw.md) — Draws text using the same logic as the [`Text`](../../advanced/widget/struct.Text.md) widget.
- [`layout`](fn.layout.md) — Computes the [`Size`](../../struct.Size.md) of a [`Text`](../../advanced/widget/struct.Text.md) widget.
- [`primary`](fn.primary.md) — Text conveying some important information, like an action.
- [`secondary`](fn.secondary.md) — Text conveying some secondary information, like a footnote.
- [`success`](fn.success.md) — Text conveying some positive information, like a successful event.
- [`warning`](fn.warning.md) — Text conveying some mildly negative information, like a warning.

## Type Aliases

- [`Fragment`](type.Fragment.md) — A fragment of [`Text`](../../advanced/struct.Text.md).
- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Text`](../../advanced/widget/struct.Text.md).
- [`Text`](type.Text.md) — A bunch of text.
