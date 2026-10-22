---
title: "button"
kind: function
path: "iced_widget::button"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.button.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.button.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "760-765"
---

# `iced_widget::button`

```rust
pub fn button<'a, Message, W, Theme>(
    content: W,
) -> Button<'a, Message, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Button`](button/struct.Button.md) with the provided content.

## Example

```rust
use iced::widget::button;

#[derive(Clone)]
enum Message {
    ButtonPressed,
}

fn view(state: &State) -> impl Widget<Message> {
    button("Press me!").on_press(Message::ButtonPressed)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 760-765)
