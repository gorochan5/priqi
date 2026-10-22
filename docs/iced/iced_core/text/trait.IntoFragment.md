---
title: "IntoFragment"
kind: trait
path: "iced_core::text::IntoFragment"
crate: "iced_core"
module: "iced_core::text"
file: "iced_core/text/trait.IntoFragment.md"
docs_url: "https://docs.iced.rs/iced_core/text/trait.IntoFragment.html"
source_file: "src/iced_core/text.rs"
source_lines: "658-661"
---

# `iced_core::text::IntoFragment`

```rust
pub trait IntoFragment<'a> {
    // Required method
    fn into_fragment(self) -> Fragment<'a>;
}
```

A trait for converting a value to some text [`Fragment`](type.Fragment.md).

**Source:** [text.rs](../../src/iced_core/text.rs.md) (lines 658-661)

## Required Methods

### `into_fragment`

```rust
fn into_fragment(self) -> Fragment<'a>
```

Converts the value to some text [`Fragment`](type.Fragment.md).

## Implementors

- `impl<'a> IntoFragment<'a> for &'a Fragment<'_>`
- `impl<'a> IntoFragment<'a> for Fragment<'a>`
