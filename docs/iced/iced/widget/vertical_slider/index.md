---
title: "vertical_slider"
kind: module
path: "iced::widget::vertical_slider"
crate: "iced"
module: "iced::widget"
file: "iced/widget/vertical_slider/index.md"
docs_url: "https://docs.iced.rs/iced/widget/vertical_slider/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "49"
---

# `iced::widget::vertical_slider`

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

## Structs

- [`Handle`](struct.Handle.md) — The appearance of the handle of a slider.
- [`Style`](struct.Style.md) — The appearance of a slider.
- [`VerticalSlider`](struct.VerticalSlider.md) — An vertical bar and a handle that selects a single value from a range of values.

## Enums

- [`HandleShape`](enum.HandleShape.md) — The shape of the handle of a slider.
- [`Status`](enum.Status.md) — The possible status of a [`Slider`](../struct.Slider.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Slider`](../struct.Slider.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`Slider`](../struct.Slider.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Slider`](../struct.Slider.md).
