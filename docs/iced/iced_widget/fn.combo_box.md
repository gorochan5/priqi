---
title: "combo_box"
kind: function
path: "iced_widget::combo_box"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.combo_box.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.combo_box.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1389-1400"
---

# `iced_widget::combo_box`

```rust
pub fn combo_box<'a, T, Message, Theme>(
    state: &'a State<T>,
    placeholder: impl IntoFragment<'a>,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> ComboBox<'a, T, Message, Theme> where
    T: Display + Clone,
    Theme: Catalog + 'a,
```

Creates a new [`ComboBox`](combo_box/struct.ComboBox.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1389-1400)
