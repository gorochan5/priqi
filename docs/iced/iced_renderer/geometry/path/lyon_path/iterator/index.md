---
title: "iterator"
kind: module
path: "iced_renderer::geometry::path::lyon_path::iterator"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/iterator/index.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/iterator/index.html"
---

# `iced_renderer::geometry::path::lyon_path::iterator`

Tools to iterate over paths.

## Lyon path iterators

### Overview

This module provides a collection of traits to extend the `Iterator` trait when iterating over paths.

### Examples

```rust
use lyon_path::iterator::*;
use lyon_path::math::{point, vector};
use lyon_path::{Path, PathEvent};

// Start with a path.
let mut builder = Path::builder();
builder.begin(point(0.0, 0.0));
builder.line_to(point(10.0, 0.0));
builder.cubic_bezier_to(point(10.0, 10.0), point(0.0, 10.0), point(0.0, 5.0));
builder.end(true);
let path = builder.build();

// A simple std::iter::Iterator<PathEvent>,
let simple_iter = path.iter();

// Make it an iterator over simpler primitives flattened events,
// which do not contain any curve. To do so we approximate each curve
// linear segments according to a tolerance threshold which controls
// the tradeoff between fidelity of the approximation and amount of
// generated events. Let's use a tolerance threshold of 0.01.
// The beauty of this approach is that the flattening happens lazily
// while iterating without allocating memory for the path.
let flattened_iter = path.iter().flattened(0.01);

for evt in flattened_iter {
    match evt {
        PathEvent::Begin { at } => { println!(" - move to {:?}", at); }
        PathEvent::Line { from, to } => { println!(" - line {:?} -> {:?}", from, to); }
        PathEvent::End { last, first, close } => {
            if close {
                println!(" - close {:?} -> {:?}", last, first);
            } else {
                println!(" - end");
            }
        }
        _ => { panic!() }
    }
}
```

Chaining the provided iterators allow performing some path manipulations lazily without allocating actual path objects to hold the result of the transformations.

```rust
extern crate lyon_path;
use lyon_path::iterator::*;
use lyon_path::math::{point, Angle, Rotation};
use lyon_path::Path;

fn main() {
    // In practice it is more common to iterate over Path objects than vectors
    // of SVG commands (the former can be constructed from the latter).
    let mut builder = Path::builder();
    builder.begin(point(1.0, 1.0));
    builder.line_to(point(2.0, 1.0));
    builder.quadratic_bezier_to(point(2.0, 2.0), point(1.0, 2.0));
    builder.cubic_bezier_to(point(0.0, 2.0), point(0.0, 0.0), point(1.0, 0.0));
    builder.end(true);
    let path = builder.build();

    let transform = Rotation::new(Angle::radians(1.0));

    for evt in path.iter().transformed(&transform).flattened(0.1) {
        // ...
    }
}
```

## Structs

- [`Flattened`](struct.Flattened.md) — An iterator that consumes `Event` iterator and yields flattened path events (with no curves).
- [`FromPolyline`](struct.FromPolyline.md) — An iterator that consumes an iterator of `Point`s and produces `Event`s.
- [`NoAttributes`](struct.NoAttributes.md)
- [`Transformed`](struct.Transformed.md) — Applies a 2D transform to a path iterator and yields the resulting path iterator.

## Traits

- [`PathIterator`](trait.PathIterator.md) — An extension trait for `PathEvent` iterators.
