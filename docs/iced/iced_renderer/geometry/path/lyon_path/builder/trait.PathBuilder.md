---
title: "PathBuilder"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::builder::PathBuilder"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/trait.PathBuilder.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/trait.PathBuilder.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::PathBuilder`

```rust
pub trait PathBuilder {
Show 20 methods    // Required methods
    fn num_attributes(&self) -> usize;
    fn begin(
        &mut self,
        at: Point2D<f32, UnknownUnit>,
        custom_attributes: &[f32],
    ) -> EndpointId;
    fn end(&mut self, close: bool);
    fn line_to(
        &mut self,
        to: Point2D<f32, UnknownUnit>,
        custom_attributes: &[f32],
    ) -> EndpointId;
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point2D<f32, UnknownUnit>,
        to: Point2D<f32, UnknownUnit>,
        custom_attributes: &[f32],
    ) -> EndpointId;
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point2D<f32, UnknownUnit>,
        ctrl2: Point2D<f32, UnknownUnit>,
        to: Point2D<f32, UnknownUnit>,
        custom_attributes: &[f32],
    ) -> EndpointId;

    // Provided methods
    fn close(&mut self) { ... }
    fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize) { ... }
    fn path_event(
        &mut self,
        event: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>,
        attributes: &[f32],
    ) { ... }
    fn event(
        &mut self,
        event: Event<(Point2D<f32, UnknownUnit>, &[f32]), Point2D<f32, UnknownUnit>>,
    ) { ... }
    fn add_polygon(
        &mut self,
        polygon: Polygon<'_, Point2D<f32, UnknownUnit>>,
        attributes: &[f32],
    ) { ... }
    fn add_point(
        &mut self,
        at: Point2D<f32, UnknownUnit>,
        attributes: &[f32],
    ) -> EndpointId { ... }
    fn add_line_segment(
        &mut self,
        line: &LineSegment<f32>,
        attributes: &[f32],
    ) -> (EndpointId, EndpointId) { ... }
    fn add_ellipse(
        &mut self,
        center: Point2D<f32, UnknownUnit>,
        radii: Vector2D<f32, UnknownUnit>,
        x_rotation: Angle<f32>,
        winding: Winding,
        attributes: &[f32],
    ) { ... }
    fn add_circle(
        &mut self,
        center: Point2D<f32, UnknownUnit>,
        radius: f32,
        winding: Winding,
        attributes: &[f32],
    )
       where Self: Sized { ... }
    fn add_rectangle(
        &mut self,
        rect: &Box2D<f32, UnknownUnit>,
        winding: Winding,
        attributes: &[f32],
    ) { ... }
    fn add_rounded_rectangle(
        &mut self,
        rect: &Box2D<f32, UnknownUnit>,
        radii: &BorderRadii,
        winding: Winding,
        custom_attributes: &[f32],
    )
       where Self: Sized { ... }
    fn flattened(self, tolerance: f32) -> Flattened<Self>
       where Self: Sized { ... }
    fn transformed<Transform>(
        self,
        transform: Transform,
    ) -> Transformed<Self, Transform>
       where Self: Sized,
             Transform: Transformation<f32> { ... }
    fn with_svg(self) -> WithSvg<Self>
       where Self: Sized { ... }
}
```

The base path building interface.

Unlike `SvgPathBuilder`, this interface strictly requires sub-paths to be manually started and ended (See the `begin` and `end` methods). All positions are provided in absolute coordinates.

The goal of this interface is to abstract over simple and fast implementations that do not deal with corner cases such as adding segments without starting a sub-path.

More elaborate interfaces are built on top of the provided primitives. In particular, the `SvgPathBuilder` trait providing more permissive and richer interface is automatically implemented via the `WithSvg` adapter (See the `with_svg` method).

## Required Methods

### `num_attributes`

```rust
fn num_attributes(&self) -> usize
```

### `begin`

```rust
fn begin( &mut self, at: Point2D<f32, UnknownUnit>, custom_attributes: &[f32], ) -> EndpointId
```

Starts a new sub-path at a given position.

There must be no sub-path in progress when this method is called. `at` becomes the current position of the sub-path.

### `end`

```rust
fn end(&mut self, close: bool)
```

Ends the current sub path.

A sub-path must be in progress when this method is called. After this method is called, there is no sub-path in progress until `begin` is called again.

### `line_to`

```rust
fn line_to( &mut self, to: Point2D<f32, UnknownUnit>, custom_attributes: &[f32], ) -> EndpointId
```

Adds a line segment to the current sub-path.

A sub-path must be in progress when this method is called.

### `quadratic_bezier_to`

```rust
fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, custom_attributes: &[f32], ) -> EndpointId
```

Adds a quadratic bézier curve to the current sub-path.

A sub-path must be in progress when this method is called.

### `cubic_bezier_to`

```rust
fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, custom_attributes: &[f32], ) -> EndpointId
```

Adds a cubic bézier curve to the current sub-path.

A sub-path must be in progress when this method is called.

## Provided Methods

### `close`

```rust
fn close(&mut self)
```

Closes the current sub path.

Shorthand for `builder.end(true)`.

### `reserve`

```rust
fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize)
```

Hints at the builder that a certain number of endpoints and control points will be added.

The Builder implementation may use this information to pre-allocate memory as an optimization.

### `path_event`

```rust
fn path_event( &mut self, event: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>, attributes: &[f32], )
```

Applies the provided path event.

By default this calls one of `begin`, `end`, `line`, `quadratic_bezier_segment`, or `cubic_bezier_segment` according to the path event.

The requirements for each method apply to the corresponding event.

### `event`

```rust
fn event( &mut self, event: Event<(Point2D<f32, UnknownUnit>, &[f32]), Point2D<f32, UnknownUnit>>, )
```

### `add_polygon`

```rust
fn add_polygon( &mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>, attributes: &[f32], )
```

Adds a sub-path from a polygon.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_point`

```rust
fn add_point( &mut self, at: Point2D<f32, UnknownUnit>, attributes: &[f32], ) -> EndpointId
```

Adds a sub-path containing a single point.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_line_segment`

```rust
fn add_line_segment( &mut self, line: &LineSegment<f32>, attributes: &[f32], ) -> (EndpointId, EndpointId)
```

Adds a sub-path containing a single line segment.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_ellipse`

```rust
fn add_ellipse( &mut self, center: Point2D<f32, UnknownUnit>, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, winding: Winding, attributes: &[f32], )
```

Adds a sub-path containing an ellipse.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_circle`

```rust
fn add_circle( &mut self, center: Point2D<f32, UnknownUnit>, radius: f32, winding: Winding, attributes: &[f32], ) where Self: Sized,
```

Adds a sub-path containing a circle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_rectangle`

```rust
fn add_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, winding: Winding, attributes: &[f32], )
```

Adds a sub-path containing a rectangle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `add_rounded_rectangle`

```rust
fn add_rounded_rectangle( &mut self, rect: &Box2D<f32, UnknownUnit>, radii: &BorderRadii, winding: Winding, custom_attributes: &[f32], ) where Self: Sized,
```

Adds a sub-path containing a rectangle.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

### `flattened`

```rust
fn flattened(self, tolerance: f32) -> Flattened<Self> where Self: Sized,
```

Returns a builder that approximates all curves with sequences of line segments.

### `transformed`

```rust
fn transformed<Transform>( self, transform: Transform, ) -> Transformed<Self, Transform> where Self: Sized, Transform: Transformation<f32>,
```

Returns a builder that applies the given transformation to all positions.

### `with_svg`

```rust
fn with_svg(self) -> WithSvg<Self> where Self: Sized,
```

Returns a builder that support SVG commands.

This must be called before starting to add any sub-path.

## Implementors

- `impl PathBuilder for BuilderImpl`
- `impl PathBuilder for iced_renderer::geometry::path::lyon_path::BuilderWithAttributes`
- `impl<'l> PathBuilder for Builder<'l>`
- `impl<'l> PathBuilder for iced_renderer::geometry::path::lyon_path::path_buffer::BuilderWithAttributes<'l>`
- `impl<B> PathBuilder for NoAttributes<B> where B: PathBuilder,`
- `impl<Builder> PathBuilder for Flattened<Builder> where Builder: PathBuilder,`
- `impl<Builder, Transform> PathBuilder for Transformed<Builder, Transform> where Builder: PathBuilder, Transform: Transformation<f32>,`
