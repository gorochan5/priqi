---
title: "StyleFn"
kind: type
path: "iced_widget::pick_list::StyleFn"
crate: "iced_widget"
module: "iced_widget::pick_list"
file: "iced_widget/pick_list/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/pick_list/type.StyleFn.html"
source_file: "src/iced_widget/pick_list.rs"
source_lines: "893"
---

# `iced_widget::pick_list::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`PickList`](struct.PickList.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [pick_list.rs](../../src/iced_widget/pick_list.rs.md) (lines 893)
