---
title: "text_input"
kind: module
path: "iced::widget::text_input"
crate: "iced"
module: "iced::widget"
file: "iced/widget/text_input/index.md"
docs_url: "https://docs.iced.rs/iced/widget/text_input/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "45"
---

# `iced::widget::text_input`

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

## Structs

- [`Style`](struct.Style.md) — The appearance of a text input.
- [`TextInput`](struct.TextInput.md) — A field that can be filled with text.

## Enums

- [`Status`](enum.Status.md) — The possible status of a [`TextInput`](../struct.TextInput.md).

## Constants

- [`DEFAULT_PADDING`](constant.DEFAULT_PADDING.md) — The default [`Padding`](../../struct.Padding.md) of a [`TextInput`](../struct.TextInput.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`TextInput`](../struct.TextInput.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`TextInput`](../struct.TextInput.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`TextInput`](../struct.TextInput.md).
