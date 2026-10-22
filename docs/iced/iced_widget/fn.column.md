---
title: "column"
kind: function
path: "iced_widget::column"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.column.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.column.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "448-453"
---

# `iced_widget::column`

```rust
pub fn column<W>(children: impl IntoIterator<Item = W>) -> Column<W> where
    W: Meta,
```

Creates a new [`Column`] with the given children.

Columns distribute their children vertically.

## Example

```rust
use iced::widget::{column, text, Widget};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    column((0..5).map(|i| text!("Item {i}")))
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 448-453)
