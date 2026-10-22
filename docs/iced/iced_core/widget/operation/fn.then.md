---
title: "then"
kind: function
path: "iced_core::widget::operation::then"
crate: "iced_core"
module: "iced_core::widget::operation"
file: "iced_core/widget/operation/fn.then.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/fn.then.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "344-419"
---

# `iced_core::widget::operation::then`

```rust
pub fn then<A, B, O>(
    operation: impl Operation<A> + 'static,
    next: impl Fn(A) -> O + Send + Sync + 'static,
) -> impl Operation<B> where
    A: 'static,
    B: 'static,
    O: Operation<B> + 'static,
```

Chains the output of an [`Operation`](trait.Operation.md) with the provided function to build a new [`Operation`](trait.Operation.md).

**Source:** [operation.rs](../../../src/iced_core/widget/operation.rs.md) (lines 344-419)
