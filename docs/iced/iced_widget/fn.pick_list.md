---
title: "pick_list"
kind: function
path: "iced_widget::pick_list"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.pick_list.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.pick_list.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1317-1330"
---

# `iced_widget::pick_list`

```rust
pub fn pick_list<'a, T, L, V, Message, Theme>(
    selected: Option<V>,
    options: L,
    to_string: impl Fn(&T) -> String + 'a,
) -> PickList<'a, T, L, V, Message, Theme> where
    T: PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: Catalog + Catalog,
```

Creates a new [`PickList`](pick_list/struct.PickList.md).

Pick lists display a dropdown list of selectable options.

## Example

```rust
use iced::widget::pick_list;

struct State {
   favorite: Option<Fruit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    let fruits = [
        Fruit::Apple,
        Fruit::Orange,
        Fruit::Strawberry,
        Fruit::Tomato,
    ];

    pick_list(
        state.favorite,
        fruits,
        Fruit::to_string,
    )
    .on_select(Message::FruitSelected)
    .placeholder("Select your favorite fruit...")
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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1317-1330)
