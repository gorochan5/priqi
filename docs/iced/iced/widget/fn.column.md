---
title: "column"
kind: function
path: "iced::widget::column"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.column.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.column.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "448-450"
---

# `iced::widget::column`

```rust
pub fn column<W>(children: impl IntoIterator<Item = W>) -> Column<W> where
    W: Meta,
```

Creates a new [`Column`](struct.Column.md) with the given children.

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

**Source:** [helpers.rs](../../src/iced_widget/helpers.rs.md) (lines 448-450)
