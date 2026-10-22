---
title: "rich_text"
kind: function
path: "iced_widget::rich_text"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.rich_text.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.rich_text.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "869-877"
---

# `iced_widget::rich_text`

```rust
pub fn rich_text<'a, Link, Message, Theme>(
    spans: impl AsRef<[Span<'a, Link>]> + 'a,
) -> Rich<'a, Link, Message, Theme> where
    Link: Clone + 'static,
    Theme: Catalog + 'a,
```

Creates a new [`Rich`](text/struct.Rich.md) text widget with the provided spans.

## Example

```rust
use iced::font;
use iced::widget::{rich_text, span};
use iced::{color, never, Font};

#[derive(Debug, Clone)]
enum Message {
    LinkClicked(&'static str),
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    rich_text([
        span("I am red!").color(color!(0xff0000)),
        span(" "),
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ])
    .on_link_click(never)
    .size(20)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 869-877)
