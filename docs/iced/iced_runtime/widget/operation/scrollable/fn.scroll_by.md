---
title: "scroll_by"
kind: function
path: "iced_runtime::widget::operation::scrollable::scroll_by"
crate: "iced_runtime"
module: "iced_runtime::widget::operation::scrollable"
file: "iced_runtime/widget/operation/scrollable/fn.scroll_by.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/scrollable/fn.scroll_by.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "71-81"
---

# `iced_runtime::widget::operation::scrollable::scroll_by`

```rust
pub fn scroll_by<T>(
    id: impl Into<Id>,
    offset: AbsoluteOffset,
    animation: Animation,
) -> Task<T>
```

Scrolls the scrollable with the given [`Id`] by the provided [`AbsoluteOffset`](struct.AbsoluteOffset.md).

**Source:** [operation.rs](../../../../src/iced_runtime/widget/operation.rs.md) (lines 71-81)
