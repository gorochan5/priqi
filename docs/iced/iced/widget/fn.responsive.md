---
title: "responsive"
kind: function
path: "iced::widget::responsive"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.responsive.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.responsive.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1738"
---

# `iced::widget::responsive`

```rust
pub fn responsive<'a, W>(f: impl Fn(Size) -> W + 'a) -> Responsive<'a, W>
```

Creates a new [`Responsive`](struct.Responsive.md) widget with a closure that produces its contents.

The `view` closure will receive the maximum available space for the [`Responsive`](struct.Responsive.md) during layout. You can use this [`Size`](../struct.Size.md) to conditionally build the contents.

**Source:** [helpers.rs](../../src/iced_widget/helpers.rs.md) (lines 1738)
