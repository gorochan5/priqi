---
title: "tooltip"
kind: function
path: "iced_widget::tooltip"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.tooltip.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.tooltip.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "793-802"
---

# `iced_widget::tooltip`

```rust
pub fn tooltip<'a, W, V, Theme>(
    content: W,
    tooltip: V,
    position: Position,
) -> Tooltip<'a, W, V, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Tooltip`](tooltip/struct.Tooltip.md) for the provided content with the given [`Widget`] and [`tooltip::Position`](tooltip/enum.Position.md).

Tooltips display a hint of information over some element when hovered.

## Example

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> impl Widget<Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    )
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 793-802)
