---
title: "text_editor"
kind: function
path: "iced_widget::text_editor"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.text_editor.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.text_editor.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1152-1161"
---

# `iced_widget::text_editor`

```rust
pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a Content<Renderer>,
) -> TextEditor<'a, PlainText, Message, Theme, Renderer> where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

Creates a new [`TextEditor`](text_editor/struct.TextEditor.md).

Text editors display a multi-line text input for text editing.

## Example

```rust
use iced::widget::text_editor;

struct State {
   content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

fn view(state: &State) -> impl Widget<Message> {
    text_editor(&state.content)
        .placeholder("Type something here...")
        .on_action(Message::Edit)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }
    }
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1152-1161)
