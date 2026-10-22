---
title: "toggler"
kind: module
path: "iced_widget::toggler"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/toggler/index.md"
docs_url: "https://docs.iced.rs/iced_widget/toggler/index.html"
source_file: "src/iced_widget/toggler.rs"
source_lines: "1-611"
---

# `iced_widget::toggler`

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

## Structs

- [`Style`](struct.Style.md) — The appearance of a toggler.
- [`Toggler`](struct.Toggler.md) — A toggler widget.

## Enums

- [`Status`](enum.Status.md) — The possible status of a [`Toggler`](struct.Toggler.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Toggler`](struct.Toggler.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`Toggler`](struct.Toggler.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Toggler`](struct.Toggler.md).
