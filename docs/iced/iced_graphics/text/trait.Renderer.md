---
title: "Renderer"
kind: trait
path: "iced_graphics::text::Renderer"
crate: "iced_graphics"
module: "iced_graphics::text"
file: "iced_graphics/text/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_graphics/text/trait.Renderer.html"
source_file: "src/iced_graphics/text.rs"
source_lines: "476-479"
---

# `iced_graphics::text::Renderer`

```rust
pub trait Renderer {
    // Required method
    fn fill_raw(&mut self, raw: Raw);
}
```

A text renderer coupled to `iced_graphics`.

**Source:** [text.rs](../../src/iced_graphics/text.rs.md) (lines 476-479)

## Required Methods

### `fill_raw`

```rust
fn fill_raw(&mut self, raw: Raw)
```

Draws the given [`Raw`](struct.Raw.md) text.

## Implementors
