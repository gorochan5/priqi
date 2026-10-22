---
title: "text_editor"
kind: module
path: "iced::widget::text_editor"
crate: "iced"
module: "iced::widget"
file: "iced/widget/text_editor/index.md"
docs_url: "https://docs.iced.rs/iced/widget/text_editor/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "44"
---

# `iced::widget::text_editor`

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

## Structs

- [`Content`](struct.Content.md) — The content of a [`TextEditor`](../struct.TextEditor.md).
- [`Cursor`](struct.Cursor.md) — The range of an [`Editor`](../../advanced/text/trait.Editor.md).
- [`KeyPress`](struct.KeyPress.md) — A key press.
- [`Line`](struct.Line.md) — A line of an [`Editor`](../../advanced/text/trait.Editor.md).
- [`Style`](struct.Style.md) — The appearance of a text input.
- [`TextEditor`](struct.TextEditor.md) — A multi-line text input.

## Enums

- [`Action`](enum.Action.md) — An interaction with an [`Editor`](../../advanced/text/trait.Editor.md).
- [`Binding`](enum.Binding.md) — A binding to an action in the [`Editor`](../../advanced/text/trait.Editor.md).
- [`Edit`](enum.Edit.md) — An action that edits text.
- [`LineEnding`](enum.LineEnding.md) — The line ending of a [`Line`](struct.Line.md).
- [`Motion`](enum.Motion.md) — A cursor movement.
- [`Selection`](enum.Selection.md) — The cursor of an [`Editor`](../../advanced/text/trait.Editor.md).
- [`Status`](enum.Status.md) — The possible status of a [`TextEditor`](../struct.TextEditor.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`TextEditor`](../struct.TextEditor.md).
- [`Highlighter`](trait.Highlighter.md) — A type that describes how to highlight an `Input` with some [`Style`](../../advanced/text/highlighter/struct.Style.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`TextEditor`](../struct.TextEditor.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`TextEditor`](../struct.TextEditor.md).
