---
title: "table"
kind: function
path: "iced::widget::table"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.table.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.table.html"
source_file: "src/iced_widget/table.rs"
source_lines: "17-24"
---

# `iced::widget::table`

```rust
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer> where
    T: Clone,
    Theme: Catalog,
    Renderer: Renderer,
```

Creates a new [`Table`](table/struct.Table.md) with the given columns and rows.

Columns can be created using the [`column()`](table/fn.column.md) function, while rows can be any iterator over some data type `T`.

**Source:** [table.rs](../../src/iced_widget/table.rs.md) (lines 17-24)
