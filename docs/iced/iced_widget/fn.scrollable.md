---
title: "scrollable"
kind: function
path: "iced_widget::scrollable"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.scrollable.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.scrollable.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "707-712"
---

# `iced_widget::scrollable`

```rust
pub fn scrollable<'a, Message, W, Theme>(
    content: W,
) -> Scrollable<'a, Message, W, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Scrollable`](scrollable/struct.Scrollable.md) with the provided content.

Scrollables let users navigate an endless amount of content with a scrollbar.

## Example

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ])
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 707-712)
