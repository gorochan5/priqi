---
title: "list"
kind: function
path: "iced_graphics::damage::list"
crate: "iced_graphics"
module: "iced_graphics::damage"
file: "iced_graphics/damage/fn.list.md"
docs_url: "https://docs.iced.rs/iced_graphics/damage/fn.list.html"
source_file: "src/iced_graphics/damage.rs"
source_lines: "30-43"
---

# `iced_graphics::damage::list`

```rust
pub fn list<T>(
    previous: &[T],
    current: &[T],
    bounds: impl Fn(&T) -> Vec<Rectangle>,
    are_equal: impl Fn(&T, &T) -> bool,
) -> Vec<Rectangle>
```

Computes the damage regions given some previous and current primitives.

**Source:** [damage.rs](../../src/iced_graphics/damage.rs.md) (lines 30-43)
