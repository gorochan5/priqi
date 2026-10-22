---
title: "container"
kind: function
path: "iced_widget::container"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.container.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.container.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "239-244"
---

# `iced_widget::container`

```rust
pub fn container<'a, W, Theme>(content: W) -> Container<'a, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Container`](container/struct.Container.md) with the provided content.

Containers let you align a widget inside their boundaries.

## Example

```rust
use iced::widget::container;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    container("This text is centered inside a rounded box!")
        .padding(10)
        .center(800)
        .style(container::rounded_box)
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 239-244)
