---
title: "snap_to"
kind: function
path: "iced_runtime::widget::operation::scrollable::snap_to"
crate: "iced_runtime"
module: "iced_runtime::widget::operation::scrollable"
file: "iced_runtime/widget/operation/scrollable/fn.snap_to.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/scrollable/fn.snap_to.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "36-46"
---

# `iced_runtime::widget::operation::scrollable::snap_to`

```rust
pub fn snap_to<T>(
    id: impl Into<Id>,
    offset: impl Into<RelativeOffset<Option<f32>>>,
    animation: Animation,
) -> Task<T>
```

Snaps the scrollable with the given [`Id`] to the provided [`RelativeOffset`](struct.RelativeOffset.md).

**Source:** [operation.rs](../../../../src/iced_runtime/widget/operation.rs.md) (lines 36-46)
