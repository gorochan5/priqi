---
title: "Default"
kind: trait
path: "iced_graphics::compositor::Default"
crate: "iced_graphics"
module: "iced_graphics::compositor"
file: "iced_graphics/compositor/trait.Default.md"
docs_url: "https://docs.iced.rs/iced_graphics/compositor/trait.Default.html"
source_file: "src/iced_graphics/compositor.rs"
source_lines: "119-122"
---

# `iced_graphics::compositor::Default`

```rust
pub trait Default {
    type Compositor: Compositor<Renderer = Self>;
}
```

Defines the default compositor of a renderer.

**Source:** [compositor.rs](../../src/iced_graphics/compositor.rs.md) (lines 119-122)

## Required Associated Types

### `Compositor`

```rust
type Compositor: Compositor<Renderer = Self>
```

The compositor of the renderer.

## Implementors
