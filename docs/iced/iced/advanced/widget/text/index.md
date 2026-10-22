---
title: "text"
kind: module
path: "iced::advanced::widget::text"
crate: "iced"
module: "iced::advanced::widget"
file: "iced/advanced/widget/text/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/text/index.html"
---

# `iced::advanced::widget::text`

Text widgets display information through writing.

## Example

```rust
use iced::widget::text;
use iced::color;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    text("Hello, this is iced!")
        .size(20)
        .color(color!(0x0000ff))
}
```

## Structs

- [`Format`](struct.Format.md) — The format of some [`Text`](../struct.Text.md).
- [`Operand`](struct.Operand.md) — The state of a widget with text, operated on by [`super::Operation::text`](../trait.Operation.md).
- [`Position`](struct.Position.md) — A specific position in some [`Text`](../../struct.Text.md).
- [`Style`](struct.Style.md) — The appearance of some text.
- [`Text`](struct.Text.md) — A bunch of text.

## Enums

- [`Alignment`](enum.Alignment.md) — The alignment of some text.
- [`Ellipsis`](enum.Ellipsis.md) — The ellipsis strategy of some text.
- [`LineHeight`](enum.LineHeight.md) — The height of a line of text in a paragraph.
- [`Shaping`](enum.Shaping.md) — The shaping strategy of some text.
- [`Wrapping`](enum.Wrapping.md) — The wrapping strategy of some text.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Text`](../struct.Text.md).

## Functions

- [`base`](fn.base.md) — Text with the default base color.
- [`danger`](fn.danger.md) — Text conveying some negative information, like an error.
- [`default`](fn.default.md) — The default text styling; color is inherited.
- [`draw`](fn.draw.md) — Draws text using the same logic as the [`Text`](../struct.Text.md) widget.
- [`layout`](fn.layout.md) — Computes the [`Size`](../../../struct.Size.md) of a [`Text`](../struct.Text.md) widget.
- [`primary`](fn.primary.md) — Text conveying some important information, like an action.
- [`secondary`](fn.secondary.md) — Text conveying some secondary information, like a footnote.
- [`success`](fn.success.md) — Text conveying some positive information, like a successful event.
- [`warning`](fn.warning.md) — Text conveying some mildly negative information, like a warning.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Text`](../struct.Text.md).
