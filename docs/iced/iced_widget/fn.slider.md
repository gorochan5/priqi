---
title: "slider"
kind: function
path: "iced_widget::slider"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.slider.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.slider.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1195-1206"
---

# `iced_widget::slider`

```rust
pub fn slider<'a, T, Message, Theme>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, Theme> where
    T: Copy + PartialOrd,
    Message: Clone,
    Theme: Catalog + 'a,
```

Creates a new [`Slider`](slider/struct.Slider.md).

Sliders let users set a value by moving an indicator.

## Example

```rust
use iced::widget::slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> impl Widget<Message> {
    slider(0.0..=100.0, state.value, Message::ValueChanged)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1195-1206)
