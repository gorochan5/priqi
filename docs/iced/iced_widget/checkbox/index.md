---
title: "checkbox"
kind: module
path: "iced_widget::checkbox"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/checkbox/index.md"
docs_url: "https://docs.iced.rs/iced_widget/checkbox/index.html"
source_file: "src/iced_widget/checkbox.rs"
source_lines: "1-725"
---

# `iced_widget::checkbox`

Checkboxes can be used to let users make binary choices.

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

## Structs

- [`Checkbox`](struct.Checkbox.md) — A box that can be checked.
- [`Icon`](struct.Icon.md) — The icon in a [`Checkbox`](struct.Checkbox.md).
- [`Style`](struct.Style.md) — The style of a checkbox.

## Enums

- [`Status`](enum.Status.md) — The possible status of a [`Checkbox`](struct.Checkbox.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Checkbox`](struct.Checkbox.md).

## Functions

- [`danger`](fn.danger.md) — A danger checkbox; denoting a negative toggle.
- [`primary`](fn.primary.md) — A primary checkbox; denoting a main toggle.
- [`secondary`](fn.secondary.md) — A secondary checkbox; denoting a complementary toggle.
- [`success`](fn.success.md) — A success checkbox; denoting a positive toggle.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Checkbox`](struct.Checkbox.md).
