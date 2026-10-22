---
title: "One"
kind: trait
path: "iced_widget::canvas::path::lyon_path::geom::euclid::num::One"
crate: "iced_widget"
module: "iced_widget::canvas::path::lyon_path::geom::euclid::num"
file: "iced_widget/canvas/path/lyon_path/geom/euclid/num/trait.One.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/path/lyon_path/geom/euclid/num/trait.One.html"
---

# `iced_widget::canvas::path::lyon_path::geom::euclid::num::One`

```rust
pub trait One {
    // Required method
    fn one() -> Self;
}
```

## Required Methods

### `one`

```rust
fn one() -> Self
```

## Implementors

- `impl<T> One for T where T: One,`
- `impl<T, Src, Dst> One for Scale<T, Src, Dst> where T: One,`
