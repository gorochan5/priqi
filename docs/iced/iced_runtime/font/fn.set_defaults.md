---
title: "set_defaults"
kind: function
path: "iced_runtime::font::set_defaults"
crate: "iced_runtime"
module: "iced_runtime::font"
file: "iced_runtime/font/fn.set_defaults.md"
docs_url: "https://docs.iced.rs/iced_runtime/font/fn.set_defaults.html"
source_file: "src/iced_runtime/font.rs"
source_lines: "65-70"
---

# `iced_runtime::font::set_defaults`

```rust
pub fn set_defaults<Message>(
    font: Font,
    text_size: impl Into<Pixels>,
) -> Task<Message>
```

Sets a new default [`Font`] and text size for the running application.

**Source:** [font.rs](../../src/iced_runtime/font.rs.md) (lines 65-70)
