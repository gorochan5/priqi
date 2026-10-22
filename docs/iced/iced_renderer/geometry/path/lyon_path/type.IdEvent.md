---
title: "IdEvent"
kind: type
path: "iced_renderer::geometry::path::lyon_path::IdEvent"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/type.IdEvent.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/type.IdEvent.html"
---

# `iced_renderer::geometry::path::lyon_path::IdEvent`

```rust
pub type IdEvent = Event<EndpointId, ControlPointId>;
```

A path event representing endpoints and control points as IDs.

## Variants

### `Begin`

**Fields**

- `at: EndpointId`

### `Line`

**Fields**

- `from: EndpointId`
- `to: EndpointId`

### `Quadratic`

**Fields**

- `from: EndpointId`
- `ctrl: ControlPointId`
- `to: EndpointId`

### `Cubic`

**Fields**

- `from: EndpointId`
- `ctrl1: ControlPointId`
- `ctrl2: ControlPointId`
- `to: EndpointId`

### `End`

**Fields**

- `last: EndpointId`
- `first: EndpointId`
- `close: bool`
