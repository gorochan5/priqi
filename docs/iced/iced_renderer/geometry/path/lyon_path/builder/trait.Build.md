---
title: "Build"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::builder::Build"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/trait.Build.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/trait.Build.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::Build`

```rust
pub trait Build {
    type PathType;

    // Required method
    fn build(self) -> Self::PathType;
}
```

Builds a path.

This trait is separate from `PathBuilder` and `SvgPathBuilder` to allow them to be used as trait object (which isn’t possible when a method returns an associated type).

## Required Associated Types

### `PathType`

```rust
type PathType
```

The type of object that is created by this builder.

## Required Methods

### `build`

```rust
fn build(self) -> Self::PathType
```

Builds a path object, consuming the builder.

## Implementors

- `impl Build for BuilderImpl`
  - `type PathType = Path`
- `impl Build for iced_renderer::geometry::path::lyon_path::BuilderWithAttributes`
  - `type PathType = Path`
- `impl<'l> Build for Builder<'l>`
  - `type PathType = usize`
- `impl<'l> Build for iced_renderer::geometry::path::lyon_path::path_buffer::BuilderWithAttributes<'l>`
  - `type PathType = usize`
- `impl<B> Build for NoAttributes<B> where B: PathBuilder + Build,`
  - `type PathType = <B as Build>::PathType`
- `impl<Builder> Build for Flattened<Builder> where Builder: Build,`
  - `type PathType = <Builder as Build>::PathType`
- `impl<Builder> Build for WithSvg<Builder> where Builder: PathBuilder + Build,`
  - `type PathType = <Builder as Build>::PathType`
- `impl<Builder, Transform> Build for Transformed<Builder, Transform> where Builder: Build,`
  - `type PathType = <Builder as Build>::PathType`
