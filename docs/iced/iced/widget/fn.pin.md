---
title: "pin"
kind: function
path: "iced::widget::pin"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.pin.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.pin.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "426"
---

# `iced::widget::pin`

```rust
pub fn pin<W>(content: W) -> Pin<W>
```

Creates a new [`Pin`](struct.Pin.md) widget with the given content.

A [`Pin`](struct.Pin.md) widget positions its contents at some fixed coordinates inside of its boundaries.

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

**Source:** [helpers.rs](../../src/iced_widget/helpers.rs.md) (lines 426)
