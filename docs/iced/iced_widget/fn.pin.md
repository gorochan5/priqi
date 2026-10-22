---
title: "pin"
kind: function
path: "iced_widget::pin"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.pin.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.pin.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "426-428"
---

# `iced_widget::pin`

```rust
pub fn pin<W>(content: W) -> Pin<W>
```

Creates a new [`Pin`] widget with the given content.

A [`Pin`] widget positions its contents at some fixed coordinates inside of its boundaries.

## Example

```rust
use iced::widget::pin;
use iced::Fill;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    pin("This text is displayed at coordinates (50, 50)!")
        .x(50)
        .y(50)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 426-428)
