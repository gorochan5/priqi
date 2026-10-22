---
title: "diff"
kind: function
path: "iced_graphics::damage::diff"
crate: "iced_graphics"
module: "iced_graphics::damage"
file: "iced_graphics/damage/fn.diff.md"
docs_url: "https://docs.iced.rs/iced_graphics/damage/fn.diff.html"
source_file: "src/iced_graphics/damage.rs"
source_lines: "5-27"
---

# `iced_graphics::damage::diff`

```rust
pub fn diff<T>(
    previous: &[T],
    current: &[T],
    bounds: impl Fn(&T) -> Vec<Rectangle>,
    diff: impl Fn(&T, &T) -> Vec<Rectangle>,
) -> Vec<Rectangle>
```

Diffs the damage regions given some previous and current primitives.

**Source:** [damage.rs](../../src/iced_graphics/damage.rs.md) (lines 5-27)
