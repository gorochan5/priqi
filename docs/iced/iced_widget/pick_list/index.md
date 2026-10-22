---
title: "pick_list"
kind: module
path: "iced_widget::pick_list"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/pick_list/index.md"
docs_url: "https://docs.iced.rs/iced_widget/pick_list/index.html"
source_file: "src/iced_widget/pick_list.rs"
source_lines: "1-943"
---

# `iced_widget::pick_list`

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

## Structs

- [`Icon`](struct.Icon.md) — The icon of a [`Handle`](enum.Handle.md).
- [`PickList`](struct.PickList.md) — A widget for selecting a single value from a list of options.
- [`Style`](struct.Style.md) — The appearance of a pick list.

## Enums

- [`Handle`](enum.Handle.md) — The handle to the right side of the [`PickList`](struct.PickList.md).
- [`Status`](enum.Status.md) — The possible status of a [`PickList`](struct.PickList.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`PickList`](struct.PickList.md).

## Functions

- [`default`](fn.default.md) — The default style of the field of a [`PickList`](struct.PickList.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`PickList`](struct.PickList.md).
