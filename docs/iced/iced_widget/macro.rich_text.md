---
title: "rich_text"
kind: macro
path: "iced_widget::rich_text"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/macro.rich_text.md"
docs_url: "https://docs.iced.rs/iced_widget/macro.rich_text.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "208-215"
---

# `iced_widget::rich_text`

```rust
macro_rules! rich_text {
    () => { ... };
    ($($x:expr),+ $(,)?) => { ... };
}
```

Creates some [`Rich`](text/struct.Rich.md) text with the given spans.

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
        span(" "),
        span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
    ]
    .on_link_click(never)
    .size(20)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 208-215)
