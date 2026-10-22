---
title: "StyleFn"
kind: type
path: "iced_widget::progress_bar::StyleFn"
crate: "iced_widget"
module: "iced_widget::progress_bar"
file: "iced_widget/progress_bar/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/progress_bar/type.StyleFn.html"
source_file: "src/iced_widget/progress_bar.rs"
source_lines: "251"
---

# `iced_widget::progress_bar::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`ProgressBar`](struct.ProgressBar.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [progress_bar.rs](../../src/iced_widget/progress_bar.rs.md) (lines 251)
