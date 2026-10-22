---
title: "lazy"
kind: function
path: "iced_widget::lazy"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.lazy.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.lazy.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1788-1796"
---

# `iced_widget::lazy`

```rust
pub fn lazy<'a, W, Dependency>(
    dependency: Dependency,
    view: impl Fn(&Dependency) -> W + 'a,
) -> Lazy<'a, W, Dependency> where
    Dependency: Hash + 'a,
```

Creates a new [`Lazy`](lazy/struct.Lazy.md) widget with the given data `Dependency` and a closure that can turn this data into a widget tree.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1788-1796)
