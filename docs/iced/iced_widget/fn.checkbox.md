---
title: "checkbox"
kind: function
path: "iced_widget::checkbox"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.checkbox.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.checkbox.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "950-958"
---

# `iced_widget::checkbox`

```rust
pub fn checkbox<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer> where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

Creates a new [`Checkbox`](checkbox/struct.Checkbox.md).

## Example

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> impl Widget<Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

![Checkbox drawn by iced_wgpu](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 950-958)
