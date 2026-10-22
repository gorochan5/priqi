---
title: "color"
kind: macro
path: "iced_core::color"
crate: "iced_core"
module: "iced_core"
file: "iced_core/macro.color.md"
docs_url: "https://docs.iced.rs/iced_core/macro.color.html"
source_file: "src/iced_core/color.rs"
source_lines: "409-431"
---

# `iced_core::color`

```rust
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => { ... };
    ($r:expr, $g:expr, $b:expr, $a:expr) => { ... };
    ($hex:literal) => { ... };
    ($hex:literal, $a:expr) => { ... };
}
```

Creates a [`Color`](color/struct.Color.md) with shorter and cleaner syntax.

## Examples

```rust
assert_eq!(color!(0, 0, 0), Color::BLACK);
assert_eq!(color!(0, 0, 0, 0.0), Color::TRANSPARENT);
assert_eq!(color!(0xffffff), Color::from_rgb(1.0, 1.0, 1.0));
assert_eq!(color!(0xffffff, 0.), Color::from_rgba(1.0, 1.0, 1.0, 0.0));
assert_eq!(color!(0x0000ff), Color::from_rgba(0.0, 0.0, 1.0, 1.0));
```

**Source:** [color.rs](../src/iced_core/color.rs.md) (lines 409-431)
