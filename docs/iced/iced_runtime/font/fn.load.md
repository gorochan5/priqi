---
title: "load"
kind: function
path: "iced_runtime::font::load"
crate: "iced_runtime"
module: "iced_runtime::font"
file: "iced_runtime/font/fn.load.md"
docs_url: "https://docs.iced.rs/iced_runtime/font/fn.load.html"
source_file: "src/iced_runtime/font.rs"
source_lines: "50-57"
---

# `iced_runtime::font::load`

```rust
pub fn load(bytes: impl Into<Cow<'static, [u8]>>) -> Task<Result<(), Error>>
```

Load a font from its bytes.

**Source:** [font.rs](../../src/iced_runtime/font.rs.md) (lines 50-57)
