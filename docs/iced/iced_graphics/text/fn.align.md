---
title: "align"
kind: function
path: "iced_graphics::text::align"
crate: "iced_graphics"
module: "iced_graphics::text"
file: "iced_graphics/text/fn.align.md"
docs_url: "https://docs.iced.rs/iced_graphics/text/fn.align.html"
source_file: "src/iced_graphics/text.rs"
source_lines: "257-292"
---

# `iced_graphics::text::align`

```rust
pub fn align(
    buffer: &mut Buffer,
    font_system: &mut FontSystem,
    alignment: Alignment,
) -> Size
```

Aligns the given [`cosmic_text::Buffer`] with the given [`Alignment`] and returns its minimum [`Size`].

**Source:** [text.rs](../../src/iced_graphics/text.rs.md) (lines 257-292)
