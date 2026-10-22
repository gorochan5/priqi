---
title: "snap_to"
kind: function
path: "iced_core::widget::operation::scrollable::snap_to"
crate: "iced_core"
module: "iced_core::widget::operation::scrollable"
file: "iced_core/widget/operation/scrollable/fn.snap_to.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/scrollable/fn.snap_to.html"
source_file: "src/iced_core/widget/operation/scrollable.rs"
source_lines: "38-73"
---

# `iced_core::widget::operation::scrollable::snap_to`

```rust
pub fn snap_to<T>(
    target: Id,
    offset: RelativeOffset<Option<f32>>,
    animation: Animation,
) -> impl Operation<T>
```

Produces an [`Operation`](../trait.Operation.md) that snaps the widget with the given [`Id`](../../struct.Id.md) to the provided `percentage`.

**Source:** [scrollable.rs](../../../../src/iced_core/widget/operation/scrollable.rs.md) (lines 38-73)
