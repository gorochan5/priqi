---
title: "combo_box"
kind: module
path: "iced::widget::combo_box"
crate: "iced"
module: "iced::widget"
file: "iced/widget/combo_box/index.md"
docs_url: "https://docs.iced.rs/iced/widget/combo_box/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "23"
---

# `iced::widget::combo_box`

Combo boxes display a dropdown list of searchable and selectable options.

## Example

```rust
use iced::widget::combo_box;

struct State {
   fruits: combo_box::State<Fruit>,
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

#[derive(Debug, Clone)]
enum Message {
    FruitSelected(Fruit),
}

fn view(state: &State) -> impl Widget<Message> {
    combo_box(
        &state.fruits,
        "Select your favorite fruit...",
        state.favorite.as_ref(),
        Message::FruitSelected
    )
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::FruitSelected(fruit) => {
            state.favorite = Some(fruit);
        }
    }
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Orange => "Orange",
            Self::Strawberry => "Strawberry",
            Self::Tomato => "Tomato",
        })
    }
}
```

## Structs

- [`ComboBox`](struct.ComboBox.md) — A widget for searching and selecting a single value from a list of options.
- [`State`](struct.State.md) — The local state of a [`ComboBox`](../struct.ComboBox.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`ComboBox`](../struct.ComboBox.md).
