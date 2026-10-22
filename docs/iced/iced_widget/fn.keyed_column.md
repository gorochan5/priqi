---
title: "keyed_column"
kind: function
path: "iced_widget::keyed_column"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.keyed_column.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.keyed_column.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "476-482"
---

# `iced_widget::keyed_column`

```rust
pub fn keyed_column<Key, W>(
    children: impl IntoIterator<Item = (Key, W)>,
) -> Column<Key, W> where
    Key: Copy + PartialEq,
    W: Meta,
```

Creates a new [`keyed::Column`](keyed/column/struct.Column.md) from an iterator of elements.

Keyed columns distribute content vertically while keeping continuity.

## Example

```rust
use iced::widget::{keyed_column, text};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    keyed_column((0..=100).map(|i| {
        (i, text!("Item {i}"))
    }))
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 476-482)
