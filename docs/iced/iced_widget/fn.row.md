---
title: "row"
kind: function
path: "iced_widget::row"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.row.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.row.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "502-507"
---

# `iced_widget::row`

```rust
pub fn row<W>(children: impl IntoIterator<Item = W>) -> Row<W> where
    W: Meta,
```

Creates a new [`Row`](row/struct.Row.md) from an iterator.

Rows distribute their children horizontally.

## Example

```rust
use iced::widget::{row, text, Widget};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    row((0..5).map(|i| text!("Item {i}")))
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 502-507)
