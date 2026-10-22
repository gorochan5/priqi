---
title: "sticky"
kind: module
path: "iced::widget::sticky"
crate: "iced"
module: "iced::widget"
file: "iced/widget/sticky/index.md"
docs_url: "https://docs.iced.rs/iced/widget/sticky/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "41"
---

# `iced::widget::sticky`

Stick elements to the viewport!

[`Sticky`](../struct.Sticky.md) keeps its contents visible when they would otherwise go out of view, for example when they are scrolled out of a [`Scrollable`](../struct.Scrollable.md). As soon as any of the contents’ edges goes out of the visible bounds, they are moved into an [`overlay`](../../advanced/overlay/index.md) inside the visible bounds, so that they stay at the edge of the viewport.

While they are stuck, the contents keep their original layout: they simply float in a new position, inside the visible bounds.

When the edges of the contents reach the edges of their parent, they stay attached to them: they keep their original bounds, and are clipped to the visible bounds instead.

Once the parent of the contents has gone out of the visible bounds, they stop floating and scroll with it.

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

## Structs

- [`Sticky`](struct.Sticky.md) — A widget that keeps its contents in view.
