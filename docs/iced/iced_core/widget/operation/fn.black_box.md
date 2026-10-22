---
title: "black_box"
kind: function
path: "iced_core::widget::operation::black_box"
crate: "iced_core"
module: "iced_core::widget::operation"
file: "iced_core/widget/operation/fn.black_box.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/fn.black_box.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "152-208"
---

# `iced_core::widget::operation::black_box`

```rust
pub fn black_box<'a, T, O>(
    operation: &'a mut dyn Operation<T>,
) -> impl Operation<O> + 'awhere
    T: 'a,
```

Wraps the [`Operation`](trait.Operation.md) in a black box, erasing its returning type.

**Source:** [operation.rs](../../../src/iced_core/widget/operation.rs.md) (lines 152-208)
