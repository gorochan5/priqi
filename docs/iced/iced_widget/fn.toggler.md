---
title: "toggler"
kind: function
path: "iced_widget::toggler"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.toggler.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.toggler.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1067-1072"
---

# `iced_widget::toggler`

```rust
pub fn toggler<'a, Message, Theme>(
    is_checked: bool,
) -> Toggler<'a, Message, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Toggler`](toggler/struct.Toggler.md).

Togglers let users make binary choices by toggling a switch.

## Example

```rust
use iced::widget::toggler;

struct State {
   is_checked: bool,
}

enum Message {
    TogglerToggled(bool),
}

fn view(state: &State) -> impl Widget<Message> {
    toggler(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::TogglerToggled)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::TogglerToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1067-1072)
