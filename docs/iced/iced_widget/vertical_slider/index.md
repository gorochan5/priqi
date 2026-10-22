---
title: "vertical_slider"
kind: module
path: "iced_widget::vertical_slider"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/vertical_slider/index.md"
docs_url: "https://docs.iced.rs/iced_widget/vertical_slider/index.html"
source_file: "src/iced_widget/vertical_slider.rs"
source_lines: "1-531"
---

# `iced_widget::vertical_slider`

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

## Re-exports

- [`Catalog`](../slider/trait.Catalog.md)
- [`Handle`](../slider/struct.Handle.md)
- [`HandleShape`](../slider/enum.HandleShape.md)
- [`Status`](../slider/enum.Status.md)
- [`Style`](../slider/struct.Style.md)
- [`StyleFn`](../slider/type.StyleFn.md)
- [`default`](../slider/fn.default.md)

## Structs

- [`VerticalSlider`](struct.VerticalSlider.md) — An vertical bar and a handle that selects a single value from a range of values.
