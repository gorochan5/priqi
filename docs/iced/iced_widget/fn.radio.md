---
title: "radio"
kind: function
path: "iced_widget::radio"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.radio.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.radio.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1020-1032"
---

# `iced_widget::radio`

```rust
pub fn radio<'a, Message, Theme, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, Theme> where
    Message: Clone,
    Theme: Catalog + 'a,
    V: Copy + Eq,
```

Creates a new [`Radio`](radio/struct.Radio.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1020-1032)
