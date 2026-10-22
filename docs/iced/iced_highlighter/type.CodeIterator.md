---
title: "CodeIterator"
kind: type
path: "iced_highlighter::CodeIterator"
crate: "iced_highlighter"
module: "iced_highlighter"
file: "iced_highlighter/type.CodeIterator.md"
docs_url: "https://docs.iced.rs/iced_highlighter/type.CodeIterator.html"
source_file: "src/iced_highlighter/lib.rs"
source_lines: "29"
---

# `iced_highlighter::CodeIterator`

```rust
pub type CodeIterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Code)> + 'a>;
```

An iterator over the highlighted regions of a line.

Each item is a character range within the line, paired with the [`Code`] of the region.

**Source:** [lib.rs](../src/iced_highlighter/lib.rs.md) (lines 29)
