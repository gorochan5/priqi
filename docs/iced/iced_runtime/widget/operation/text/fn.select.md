---
title: "select"
kind: function
path: "iced_runtime::widget::operation::text::select"
crate: "iced_runtime"
module: "iced_runtime::widget::operation::text"
file: "iced_runtime/widget/operation/text/fn.select.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/text/fn.select.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "94-96"
---

# `iced_runtime::widget::operation::text::select`

```rust
pub fn select<T>(start: Point, end: Point, target: Target) -> Task<T>
```

Selects text between the given positions in layout coordinates.

`start` is the position where the user started selecting from, while `end` is the final position of the selection.

**Source:** [operation.rs](../../../../src/iced_runtime/widget/operation.rs.md) (lines 94-96)
