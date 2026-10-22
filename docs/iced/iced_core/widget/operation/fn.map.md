---
title: "map"
kind: function
path: "iced_core::widget::operation::map"
crate: "iced_core"
module: "iced_core::widget::operation"
file: "iced_core/widget/operation/fn.map.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/fn.map.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "211-340"
---

# `iced_core::widget::operation::map`

```rust
pub fn map<A, B>(
    operation: impl Operation<A>,
    f: impl Fn(A) -> B + Send + Sync + 'static,
) -> impl Operation<B> where
    A: 'static,
    B: 'static,
```

Maps the output of an [`Operation`](trait.Operation.md) using the given function.

**Source:** [operation.rs](../../../src/iced_core/widget/operation.rs.md) (lines 211-340)
