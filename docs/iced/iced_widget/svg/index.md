---
title: "svg"
kind: module
path: "iced_widget::svg"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/svg/index.md"
docs_url: "https://docs.iced.rs/iced_widget/svg/index.html"
source_file: "src/iced_widget/svg.rs"
source_lines: "1-323"
---

# `iced_widget::svg`

Svg widgets display vector graphics in your application.

## Example

```rust
use iced::widget::svg;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    svg("tiger.svg")
}
```

## Structs

- [`Handle`](struct.Handle.md) — A handle of Svg data.
- [`Style`](struct.Style.md) — The appearance of an [`Svg`](struct.Svg.md).
- [`Svg`](struct.Svg.md) — A vector graphics image.

## Enums

- [`Status`](enum.Status.md) — The possible status of an [`Svg`](struct.Svg.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of an [`Svg`](struct.Svg.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for an [`Svg`](struct.Svg.md).
