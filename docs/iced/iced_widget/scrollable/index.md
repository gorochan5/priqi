---
title: "scrollable"
kind: module
path: "iced_widget::scrollable"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/scrollable/index.md"
docs_url: "https://docs.iced.rs/iced_widget/scrollable/index.html"
source_file: "src/iced_widget/scrollable.rs"
source_lines: "1-3650"
---

# `iced_widget::scrollable`

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

## Structs

- [`AbsoluteOffset`](struct.AbsoluteOffset.md) — The amount of absolute offset in each direction of a [`Scrollable`].
- [`AutoScroll`](struct.AutoScroll.md) — The appearance of the autoscroll overlay of a scrollable.
- [`Rail`](struct.Rail.md) — The appearance of the scrollbar of a scrollable.
- [`RelativeOffset`](struct.RelativeOffset.md) — The amount of relative offset in each direction of a [`Scrollable`].
- [`Scroll`](struct.Scroll.md) — An event describing a scroll of a [`Scrollable`](struct.Scrollable.md).
- [`Scrollable`](struct.Scrollable.md) — A widget that can vertically display an infinite amount of content with a scrollbar.
- [`Scrollbar`](struct.Scrollbar.md) — A scrollbar within a [`Scrollable`](struct.Scrollable.md).
- [`Scroller`](struct.Scroller.md) — The appearance of the scroller of a scrollable.
- [`Style`](struct.Style.md) — The appearance of a scrollable.
- [`Target`](struct.Target.md) — The target of an in-progress smooth scroll.
- [`Viewport`](struct.Viewport.md) — The current [`Viewport`](struct.Viewport.md) of the [`Scrollable`](struct.Scrollable.md).

## Enums

- [`Action`](enum.Action.md) — An action to perform in response to a [`Scroll`](struct.Scroll.md).
- [`Anchor`](enum.Anchor.md) — The anchor of the scroller of the [`Scrollable`](struct.Scrollable.md) relative to its [`Viewport`](struct.Viewport.md) on a given axis.
- [`Direction`](enum.Direction.md) — The direction of [`Scrollable`](struct.Scrollable.md).
- [`Offset`](enum.Offset.md) — An offset of a [`Scrollable`](struct.Scrollable.md), in either absolute or relative units.
- [`Source`](enum.Source.md) — The source of a scroll notification.
- [`Status`](enum.Status.md) — The possible status of a [`Scrollable`](struct.Scrollable.md).

## Traits

- [`Catalog`](trait.Catalog.md) — The theme catalog of a [`Scrollable`](struct.Scrollable.md).

## Functions

- [`default`](fn.default.md) — The default style of a [`Scrollable`](struct.Scrollable.md).

## Type Aliases

- [`StyleFn`](type.StyleFn.md) — A styling function for a [`Scrollable`](struct.Scrollable.md).
