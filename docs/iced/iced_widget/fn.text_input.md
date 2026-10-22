---
title: "text_input"
kind: function
path: "iced_widget::text_input"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.text_input.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.text_input.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1107-1116"
---

# `iced_widget::text_input`

```rust
pub fn text_input<'a, Message, Theme>(
    placeholder: impl IntoFragment<'a>,
    value: impl IntoFragment<'a>,
) -> TextInput<'a, Message, Theme> where
    Message: Clone,
    Theme: Catalog + 'a,
```

Creates a new [`TextInput`](text_input/struct.TextInput.md).

Text inputs display fields that can be filled with text.

## Example

```rust
use iced::widget::text_input;

struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> impl Widget<Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1107-1116)
