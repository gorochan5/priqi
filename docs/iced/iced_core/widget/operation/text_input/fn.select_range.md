---
title: "select_range"
kind: function
path: "iced_core::widget::operation::text_input::select_range"
crate: "iced_core"
module: "iced_core::widget::operation::text_input"
file: "iced_core/widget/operation/text_input/fn.select_range.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/text_input/fn.select_range.html"
source_file: "src/iced_core/widget/operation/text_input.rs"
source_lines: "131-158"
---

# `iced_core::widget::operation::text_input::select_range`

```rust
pub fn select_range<T>(
    target: Id,
    start: Position,
    end: Position,
) -> impl Operation<T>
```

Produces an [`Operation`](../trait.Operation.md) that selects the given content range of the widget with the given [`Id`](../../struct.Id.md).

**Source:** [text_input.rs](../../../../src/iced_core/widget/operation/text_input.rs.md) (lines 131-158)
