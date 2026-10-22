---
title: "Cached"
kind: trait
path: "iced_graphics::cache::Cached"
crate: "iced_graphics"
module: "iced_graphics::cache"
file: "iced_graphics/cache/trait.Cached.md"
docs_url: "https://docs.iced.rs/iced_graphics/cache/trait.Cached.html"
source_file: "src/iced_graphics/cache.rs"
source_lines: "174-187"
---

# `iced_graphics::cache::Cached`

```rust
pub trait Cached: Sized {
    type Cache: Clone;

    // Required methods
    fn load(cache: &Self::Cache) -> Self;
    fn cache(self, group: Group, previous: Option<Self::Cache>) -> Self::Cache;
}
```

A piece of data that can be cached.

**Source:** [cache.rs](../../src/iced_graphics/cache.rs.md) (lines 174-187)

## Required Associated Types

### `Cache`

```rust
type Cache: Clone
```

The type of cache produced.

## Required Methods

### `load`

```rust
fn load(cache: &Self::Cache) -> Self
```

Loads the [`Cache`](trait.Cached.md) into a proper instance.

### `cache`

```rust
fn cache(self, group: Group, previous: Option<Self::Cache>) -> Self::Cache
```

Caches this value, producing its corresponding [`Cache`](trait.Cached.md).

## Implementors
