---
title: "responsive"
kind: function
path: "iced_widget::responsive"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.responsive.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.responsive.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1738-1740"
---

# `iced_widget::responsive`

```rust
pub fn responsive<'a, W>(f: impl Fn(Size) -> W + 'a) -> Responsive<'a, W>
```

Creates a new [`Responsive`] widget with a closure that produces its contents.

The `view` closure will receive the maximum available space for the [`Responsive`] during layout. You can use this [`Size`] to conditionally build the contents.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1738-1740)
