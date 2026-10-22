---
title: "select_range"
kind: function
path: "iced_runtime::widget::operation::text_input::select_range"
crate: "iced_runtime"
module: "iced_runtime::widget::operation::text_input"
file: "iced_runtime/widget/operation/text_input/fn.select_range.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/text_input/fn.select_range.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "123-129"
---

# `iced_runtime::widget::operation::text_input::select_range`

```rust
pub fn select_range<T>(
    id: impl Into<Id>,
    start: Position,
    end: Position,
) -> Task<T>
```

Selects the given content range of the widget with the given [`Id`].

**Source:** [operation.rs](../../../../src/iced_runtime/widget/operation.rs.md) (lines 123-129)
