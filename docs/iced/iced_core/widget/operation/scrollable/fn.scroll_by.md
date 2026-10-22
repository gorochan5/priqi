---
title: "scroll_by"
kind: function
path: "iced_core::widget::operation::scrollable::scroll_by"
crate: "iced_core"
module: "iced_core::widget::operation::scrollable"
file: "iced_core/widget/operation/scrollable/fn.scroll_by.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/scrollable/fn.scroll_by.html"
source_file: "src/iced_core/widget/operation/scrollable.rs"
source_lines: "116-147"
---

# `iced_core::widget::operation::scrollable::scroll_by`

```rust
pub fn scroll_by<T>(
    target: Id,
    offset: AbsoluteOffset,
    animation: Animation,
) -> impl Operation<T>
```

Produces an [`Operation`](../trait.Operation.md) that scrolls the widget with the given [`Id`](../../struct.Id.md) by the provided [`AbsoluteOffset`](struct.AbsoluteOffset.md).

**Source:** [scrollable.rs](../../../../src/iced_core/widget/operation/scrollable.rs.md) (lines 116-147)
