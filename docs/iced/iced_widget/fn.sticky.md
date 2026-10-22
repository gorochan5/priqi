---
title: "sticky"
kind: function
path: "iced_widget::sticky"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.sticky.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.sticky.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "738-740"
---

# `iced_widget::sticky`

```rust
pub fn sticky<W>(content: W) -> Sticky<W>
```

Creates a new [`Sticky`](sticky/struct.Sticky.md) for the provided content.

The contents of a [`Sticky`](sticky/struct.Sticky.md) will be displayed on an overlay, inside the visible bounds, whenever they would otherwise go out of view.

## Example

```rust
use iced::widget::{column, container, scrollable, sticky, space};
use iced::Fill;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    scrollable(column![
        sticky(container("I always stay in view!").width(Fill).padding(10)),
        space().height(3000),
    ])
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 738-740)
