---
title: "button"
kind: module
path: "iced_widget::button"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/button/index.md"
docs_url: "https://docs.iced.rs/iced_widget/button/index.html"
source_file: "src/iced_widget/button.rs"
source_lines: "1-726"
---

# `iced_widget::button`

Buttons allow your users to perform actions by pressing them.

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

## Structs

- [`Button`](struct.Button.md) — A generic widget that produces a message when pressed.
- [`Style`](struct.Style.md) — The style of a button.

## Enums

- [`Status`](enum.Status.md) — The possible status of a [`Button`](struct.Button.md).

## Constants

- [`DEFAULT_PADDING`](constant.DEFAULT_PADDING.md) — The default [`Padding`] of a [`Button`](struct.Button.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Button`](struct.Button.md).

## Functions

- [`background`](fn.background.md) — A button using background shades.
- [`danger`](fn.danger.md) — A danger button; denoting a destructive action.
- [`primary`](fn.primary.md) — A primary button; denoting a main action.
- [`secondary`](fn.secondary.md) — A secondary button; denoting a complementary action.
- [`subtle`](fn.subtle.md) — A subtle button using weak background shades.
- [`success`](fn.success.md) — A success button; denoting a good outcome.
- [`text`](fn.text.md) — A text button; useful for links.
- [`warning`](fn.warning.md) — A warning button; denoting a risky action.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Button`](struct.Button.md).
