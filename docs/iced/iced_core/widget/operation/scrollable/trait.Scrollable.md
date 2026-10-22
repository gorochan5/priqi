---
title: "Scrollable"
kind: trait
path: "iced_core::widget::operation::scrollable::Scrollable"
crate: "iced_core"
module: "iced_core::widget::operation::scrollable"
file: "iced_core/widget/operation/scrollable/trait.Scrollable.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/scrollable/trait.Scrollable.html"
source_file: "src/iced_core/widget/operation/scrollable.rs"
source_lines: "7-34"
---

# `iced_core::widget::operation::scrollable::Scrollable`

```rust
pub trait Scrollable {
    // Required methods
    fn snap_to(
        &mut self,
        offset: RelativeOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    );
    fn scroll_to(
        &mut self,
        offset: AbsoluteOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    );
    fn scroll_by(
        &mut self,
        offset: AbsoluteOffset,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    );
}
```

The internal state of a widget that can be scrolled.

**Source:** [scrollable.rs](../../../../src/iced_core/widget/operation/scrollable.rs.md) (lines 7-34)

## Required Methods

### `snap_to`

```rust
fn snap_to( &mut self, offset: RelativeOffset<Option<f32>>, animation: Animation, bounds: Rectangle, content: Size, )
```

Snaps the scroll of the widget to the given `percentage` along the horizontal & vertical axis.

### `scroll_to`

```rust
fn scroll_to( &mut self, offset: AbsoluteOffset<Option<f32>>, animation: Animation, bounds: Rectangle, content: Size, )
```

Scroll the widget to the given [`AbsoluteOffset`](struct.AbsoluteOffset.md) along the horizontal & vertical axis.

### `scroll_by`

```rust
fn scroll_by( &mut self, offset: AbsoluteOffset, animation: Animation, bounds: Rectangle, content: Size, )
```

Scroll the widget by the given [`AbsoluteOffset`](struct.AbsoluteOffset.md) along the horizontal & vertical axis.

## Implementors
