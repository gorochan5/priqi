---
title: "text"
kind: module
path: "iced_core::widget::text"
crate: "iced_core"
module: "iced_core::widget"
file: "iced_core/widget/text/index.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/index.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "1-707"
---

# `iced_core::widget::text`

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

## Re-exports

- [`Alignment`](../../text/enum.Alignment.md)
- [`Ellipsis`](../../text/enum.Ellipsis.md)
- [`LineHeight`](../../text/enum.LineHeight.md)
- [`Position`](../../text/struct.Position.md)
- [`Shaping`](../../text/enum.Shaping.md)
- [`Wrapping`](../../text/enum.Wrapping.md)

## Structs

- [`Format`](struct.Format.md) — The format of some [`Text`](struct.Text.md).
- [`Operand`](struct.Operand.md) — The state of a widget with text, operated on by [`super::Operation::text`](../operation/trait.Operation.md).
- [`Style`](struct.Style.md) — The appearance of some text.
- [`Text`](struct.Text.md) — A bunch of text.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Text`](struct.Text.md).

## Functions

- [`base`](fn.base.md) — Text with the default base color.
- [`danger`](fn.danger.md) — Text conveying some negative information, like an error.
- [`default`](fn.default.md) — The default text styling; color is inherited.
- [`draw`](fn.draw.md) — Draws text using the same logic as the [`Text`](struct.Text.md) widget.
- [`layout`](fn.layout.md) — Computes the [`Size`](../../struct.Size.md) of a [`Text`](struct.Text.md) widget.
- [`primary`](fn.primary.md) — Text conveying some important information, like an action.
- [`secondary`](fn.secondary.md) — Text conveying some secondary information, like a footnote.
- [`success`](fn.success.md) — Text conveying some positive information, like a successful event.
- [`warning`](fn.warning.md) — Text conveying some mildly negative information, like a warning.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Text`](struct.Text.md).
