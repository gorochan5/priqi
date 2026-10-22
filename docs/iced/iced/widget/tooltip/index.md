---
title: "tooltip"
kind: module
path: "iced::widget::tooltip"
crate: "iced"
module: "iced::widget"
file: "iced/widget/tooltip/index.md"
docs_url: "https://docs.iced.rs/iced/widget/tooltip/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "47"
---

# `iced::widget::tooltip`

Tooltips display a hint of information over some element when hovered.

By default, the tooltip is displayed immediately, however, this can be adjusted with [`Tooltip::delay`](../struct.Tooltip.md).

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

## Structs

- [`Tooltip`](struct.Tooltip.md) — An element to display a widget over another.

## Enums

- [`Position`](enum.Position.md) — The position of the tooltip. Defaults to following the cursor.
