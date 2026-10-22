---
title: "Renderer"
kind: trait
path: "iced_graphics::mesh::Renderer"
crate: "iced_graphics"
module: "iced_graphics::mesh"
file: "iced_graphics/mesh/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_graphics/mesh/trait.Renderer.html"
source_file: "src/iced_graphics/mesh.rs"
source_lines: "200-206"
---

# `iced_graphics::mesh::Renderer`

```rust
pub trait Renderer {
    // Required methods
    fn draw_mesh(&mut self, mesh: Mesh);
    fn draw_mesh_cache(&mut self, cache: Cache);
}
```

A renderer capable of drawing a [`Mesh`](enum.Mesh.md).

**Source:** [mesh.rs](../../src/iced_graphics/mesh.rs.md) (lines 200-206)

## Required Methods

### `draw_mesh`

```rust
fn draw_mesh(&mut self, mesh: Mesh)
```

Draws the given [`Mesh`](enum.Mesh.md).

### `draw_mesh_cache`

```rust
fn draw_mesh_cache(&mut self, cache: Cache)
```

Draws the given [`Cache`](struct.Cache.md).

## Implementors
