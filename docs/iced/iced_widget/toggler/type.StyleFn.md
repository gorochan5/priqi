---
title: "StyleFn"
kind: type
path: "iced_widget::toggler::StyleFn"
crate: "iced_widget"
module: "iced_widget::toggler"
file: "iced_widget/toggler/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/toggler/type.StyleFn.html"
source_file: "src/iced_widget/toggler.rs"
source_lines: "540"
---

# `iced_widget::toggler::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Toggler`](struct.Toggler.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [toggler.rs](../../src/iced_widget/toggler.rs.md) (lines 540)
