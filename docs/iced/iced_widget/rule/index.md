---
title: "rule"
kind: module
path: "iced_widget::rule"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/rule/index.md"
docs_url: "https://docs.iced.rs/iced_widget/rule/index.html"
source_file: "src/iced_widget/rule.rs"
source_lines: "1-314"
---

# `iced_widget::rule`

Rules divide space horizontally or vertically.

## Example

```rust
use iced::widget::rule;

#[derive(Clone)]
enum Message {
    // ...,
}

fn view(state: &State) -> impl Widget<Message> {
    rule::horizontal(2)
}
```

## Structs

- [`Rule`](struct.Rule.md) — Display a horizontal or vertical rule for dividing content.
- [`Style`](struct.Style.md) — The appearance of a rule.

## Enums

- [`FillMode`](enum.FillMode.md) — The fill mode of a rule.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Rule`](struct.Rule.md).

## Functions

- [`default`](fn.default.md) — The default styling of a [`Rule`](struct.Rule.md).
- [`horizontal`](fn.horizontal.md) — Creates a new horizontal [`Rule`](struct.Rule.md) with the given height.
- [`vertical`](fn.vertical.md) — Creates a new vertical [`Rule`](struct.Rule.md) with the given width.
- [`weak`](fn.weak.md) — A [`Rule`](struct.Rule.md) styling using the weak background color.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Rule`](struct.Rule.md).
