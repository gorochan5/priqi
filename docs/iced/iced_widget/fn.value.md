---
title: "value"
kind: function
path: "iced_widget::value"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.value.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.value.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "833-838"
---

# `iced_widget::value`

```rust
pub fn value<'a, Theme>(value: impl ToString) -> Text<'a, Theme> where
    Theme: Catalog + 'a,
```

Creates a new [`Text`](text/type.Text.md) widget that displays the provided value.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 833-838)
