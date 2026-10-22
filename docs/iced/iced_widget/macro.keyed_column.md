---
title: "keyed_column"
kind: macro
path: "iced_widget::keyed_column"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/macro.keyed_column.md"
docs_url: "https://docs.iced.rs/iced_widget/macro.keyed_column.html"
source_file: "src/iced_widget/keyed.rs"
source_lines: "68-75"
---

# `iced_widget::keyed_column`

```rust
macro_rules! keyed_column {
    () => { ... };
    ($(($key:expr, $x:expr)),+ $(,)?) => { ... };
}
```

Creates a keyed [`Column`](keyed/column/struct.Column.md) with the given children.

Keyed columns distribute content vertically while keeping continuity.

## Example

```rust
use iced::widget::keyed_column;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    keyed_column![
        (0, "Item 0"),
        (1, "Item 1"),
        (2, "Item 2"),
    ]
}
```

**Source:** [keyed.rs](../src/iced_widget/keyed.rs.md) (lines 68-75)
