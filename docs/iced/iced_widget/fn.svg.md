---
title: "svg"
kind: function
path: "iced_widget::svg"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.svg.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.svg.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1494-1499"
---

# `iced_widget::svg`

```rust
pub fn svg<'a, Theme>(handle: impl Into<Handle>) -> Svg<'a, Theme> where
    Theme: Catalog,
```

Creates a new [`Svg`](svg/struct.Svg.md) widget from the given [`Handle`](svg/struct.Handle.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1494-1499)
