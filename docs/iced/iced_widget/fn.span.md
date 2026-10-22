---
title: "span"
kind: function
path: "iced_widget::span"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.span.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.span.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "910-912"
---

# `iced_widget::span`

```rust
pub fn span<'a, Link>(text: impl IntoFragment<'a>) -> Span<'a, Link>
```

Creates a new [`Span`](text/struct.Span.md) of text with the provided content.

A [`Span`](text/struct.Span.md) is a fragment of some [`Rich`](text/struct.Rich.md) text.

## Example

```rust
use iced::font;
use iced::widget::{rich_text, span};
use iced::{color, never, Font};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    rich_text![
        span("I am red!").color(color!(0xff0000)),
        " ",
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ]
    .on_link_click(never)
    .size(20)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 910-912)
