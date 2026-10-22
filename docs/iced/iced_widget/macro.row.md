---
title: "row"
kind: macro
path: "iced_widget::row"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/macro.row.md"
docs_url: "https://docs.iced.rs/iced_widget/macro.row.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "97-104"
---

# `iced_widget::row`

```rust
macro_rules! row {
    () => { ... };
    ($($x:expr),+ $(,)?) => { ... };
}
```

Creates a [`Row`](row/struct.Row.md) with the given children.

Rows distribute their children horizontally.

## Example

```rust
use iced::widget::{button, row};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    row![
        "I am to the left!",
        button("I am in the middle!"),
        "I am to the right!",
    ]
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 97-104)
