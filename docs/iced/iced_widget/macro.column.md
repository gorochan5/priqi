---
title: "column"
kind: macro
path: "iced_widget::column"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/macro.column.md"
docs_url: "https://docs.iced.rs/iced_widget/macro.column.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "63-70"
---

# `iced_widget::column`

```rust
macro_rules! column {
    () => { ... };
    ($($x:expr),+ $(,)?) => { ... };
}
```

Creates a [`Column`] with the given children.

Columns distribute their children vertically.

## Example

```rust
use iced::widget::{button, column};

#[derive(Debug, Clone)]
enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    column![
        "I am on top!",
        button("I am in the center!"),
        "I am below.",
    ]
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 63-70)
