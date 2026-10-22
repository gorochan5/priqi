---
title: "select"
kind: function
path: "iced_core::widget::operation::text::select"
crate: "iced_core"
module: "iced_core::widget::operation::text"
file: "iced_core/widget/operation/text/fn.select.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/text/fn.select.html"
source_file: "src/iced_core/widget/operation/text.rs"
source_lines: "48-94"
---

# `iced_core::widget::operation::text::select`

```rust
pub fn select(start: Point, end: Point, target: Target) -> impl Operation
```

Selects text between the given positions in layout coordinates.

`start` is the position where the user started selecting from, while `end` is the final position of the selection.

**Source:** [text.rs](../../../../src/iced_core/widget/operation/text.rs.md) (lines 48-94)
