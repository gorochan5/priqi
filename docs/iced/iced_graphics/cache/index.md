---
title: "cache"
kind: module
path: "iced_graphics::cache"
crate: "iced_graphics"
module: "iced_graphics"
file: "iced_graphics/cache/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/cache/index.html"
source_file: "src/iced_graphics/cache.rs"
source_lines: "1-201"
---

# `iced_graphics::cache`

Cache computations and efficiently reuse them.

## Structs

- [`Cache`](struct.Cache.md) — A simple cache that stores generated values to avoid recomputation.
- [`Group`](struct.Group.md) — A cache group.

## Enums

- [`State`](enum.State.md) — The state of a [`Cache`](struct.Cache.md).

## Traits

- [`Cached`](trait.Cached.md) — A piece of data that can be cached.

## Functions

- [`invalidate_all`](fn.invalidate_all.md) — Invalidates all the existing [`Cache`](struct.Cache.md); clearing their contents.
