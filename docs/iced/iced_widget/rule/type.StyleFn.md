---
title: "StyleFn"
kind: type
path: "iced_widget::rule::StyleFn"
crate: "iced_widget"
module: "iced_widget::rule"
file: "iced_widget/rule/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/rule/type.StyleFn.html"
source_file: "src/iced_widget/rule.rs"
source_lines: "278"
---

# `iced_widget::rule::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Rule`](struct.Rule.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [rule.rs](../../src/iced_widget/rule.rs.md) (lines 278)
