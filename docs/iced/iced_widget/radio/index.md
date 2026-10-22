---
title: "radio"
kind: module
path: "iced_widget::radio"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/radio/index.md"
docs_url: "https://docs.iced.rs/iced_widget/radio/index.html"
source_file: "src/iced_widget/radio.rs"
source_lines: "1-534"
---

# `iced_widget::radio`

Radio buttons let users choose a single option from a bunch of options.

## Example

```rust
use iced::widget::{column, radio};

struct State {
   selection: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    A,
    B,
    C,
    All,
}

fn view(state: &State) -> impl Widget<Message> {
    let a = radio(
        "A",
        Choice::A,
        state.selection,
        Message::RadioSelected,
    );

    let b = radio(
        "B",
        Choice::B,
        state.selection,
        Message::RadioSelected,
    );

    let c = radio(
        "C",
        Choice::C,
        state.selection,
        Message::RadioSelected,
    );

    let all = radio(
        "All of the above",
        Choice::All,
        state.selection,
        Message::RadioSelected
    );

    column![a, b, c, all]
}
```

## Structs

- [`Radio`](struct.Radio.md) — A circular button representing a choice.
- [`Style`](struct.Style.md) — The appearance of a radio button.

## Enums

- [`Status`](enum.Status.md) — The possible status of a [`Radio`](struct.Radio.md) button.

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Radio`](struct.Radio.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`Radio`](struct.Radio.md) button.

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Radio`](struct.Radio.md).
