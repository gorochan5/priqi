---
title: "scope"
kind: function
path: "iced_core::widget::operation::scope"
crate: "iced_core"
module: "iced_core::widget::operation"
file: "iced_core/widget/operation/fn.scope.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/fn.scope.html"
source_file: "src/iced_core/widget/operation.rs"
source_lines: "423-462"
---

# `iced_core::widget::operation::scope`

```rust
pub fn scope<T: 'static>(
    target: Id,
    operation: impl Operation<T> + 'static,
) -> impl Operation<T>
```

Produces an [`Operation`](trait.Operation.md) that applies the given [`Operation`](trait.Operation.md) to the children of a container with the given [`Id`](../struct.Id.md).

**Source:** [operation.rs](../../../src/iced_core/widget/operation.rs.md) (lines 423-462)
