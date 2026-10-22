---
title: "column"
kind: function
path: "iced_widget::table::column"
crate: "iced_widget"
module: "iced_widget::table"
file: "iced_widget/table/fn.column.md"
docs_url: "https://docs.iced.rs/iced_widget/table/fn.column.html"
source_file: "src/iced_widget/table.rs"
source_lines: "33-48"
---

# `iced_widget::table::column`

```rust
pub fn column<'a, 'b, T, W, Message, Theme, Renderer>(
    header: impl Widget<Message, Theme, Renderer> + 'a,
    view: impl Fn(T) -> W + 'b,
) -> Column<'a, 'b, T, Message, Theme, Renderer> where
    T: 'a,
    W: Widget<Message, Theme, Renderer> + 'a,
```

Creates a new [`Column`](struct.Column.md) with the given header and view function.

The view function will be called for each row in a [`Table`](struct.Table.md) and it must produce the resulting contents of a cell.

**Source:** [table.rs](../../src/iced_widget/table.rs.md) (lines 33-48)
