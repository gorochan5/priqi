---
title: "SvgPathBuilder"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::builder::SvgPathBuilder"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::builder"
file: "iced_renderer/geometry/path/lyon_path/builder/trait.SvgPathBuilder.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/builder/trait.SvgPathBuilder.html"
---

# `iced_renderer::geometry::path::lyon_path::builder::SvgPathBuilder`

```rust
pub trait SvgPathBuilder {
Show 21 methods    // Required methods
    fn move_to(&mut self, to: Point2D<f32, UnknownUnit>);
    fn close(&mut self);
    fn line_to(&mut self, to: Point2D<f32, UnknownUnit>);
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point2D<f32, UnknownUnit>,
        to: Point2D<f32, UnknownUnit>,
    );
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point2D<f32, UnknownUnit>,
        ctrl2: Point2D<f32, UnknownUnit>,
        to: Point2D<f32, UnknownUnit>,
    );
    fn relative_move_to(&mut self, to: Vector2D<f32, UnknownUnit>);
    fn relative_line_to(&mut self, to: Vector2D<f32, UnknownUnit>);
    fn relative_quadratic_bezier_to(
        &mut self,
        ctrl: Vector2D<f32, UnknownUnit>,
        to: Vector2D<f32, UnknownUnit>,
    );
    fn relative_cubic_bezier_to(
        &mut self,
        ctrl1: Vector2D<f32, UnknownUnit>,
        ctrl2: Vector2D<f32, UnknownUnit>,
        to: Vector2D<f32, UnknownUnit>,
    );
    fn smooth_cubic_bezier_to(
        &mut self,
        ctrl2: Point2D<f32, UnknownUnit>,
        to: Point2D<f32, UnknownUnit>,
    );
    fn smooth_relative_cubic_bezier_to(
        &mut self,
        ctrl2: Vector2D<f32, UnknownUnit>,
        to: Vector2D<f32, UnknownUnit>,
    );
    fn smooth_quadratic_bezier_to(&mut self, to: Point2D<f32, UnknownUnit>);
    fn smooth_relative_quadratic_bezier_to(
        &mut self,
        to: Vector2D<f32, UnknownUnit>,
    );
    fn horizontal_line_to(&mut self, x: f32);
    fn relative_horizontal_line_to(&mut self, dx: f32);
    fn vertical_line_to(&mut self, y: f32);
    fn relative_vertical_line_to(&mut self, dy: f32);
    fn arc_to(
        &mut self,
        radii: Vector2D<f32, UnknownUnit>,
        x_rotation: Angle<f32>,
        flags: ArcFlags,
        to: Point2D<f32, UnknownUnit>,
    );
    fn relative_arc_to(
        &mut self,
        radii: Vector2D<f32, UnknownUnit>,
        x_rotation: Angle<f32>,
        flags: ArcFlags,
        to: Vector2D<f32, UnknownUnit>,
    );

    // Provided methods
    fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize) { ... }
    fn add_polygon(&mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>) { ... }
}
```

A path building interface that tries to stay close to SVG’s path specification. [https://svgwg.org/specs/paths/](https://svgwg.org/specs/paths/)

Some of the wording in the documentation of this trait is borrowed from the SVG specification.

Unlike `PathBuilder`, implementations of this trait are expected to deal with various corners cases such as adding segments without starting a sub-path.

## Required Methods

### `move_to`

```rust
fn move_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Start a new sub-path at the given position.

Corresponding SVG command: `M`.

This command establishes a new initial point and a new current point. The effect is as if the “pen” were lifted and moved to a new location. If a sub-path is in progress, it is ended without being closed.

### `close`

```rust
fn close(&mut self)
```

Ends the current sub-path by connecting it back to its initial point.

Corresponding SVG command: `Z`.

A straight line is drawn from the current point to the initial point of the current sub-path. The current position is set to the initial position of the sub-path that was closed.

### `line_to`

```rust
fn line_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Adds a line segment to the current sub-path.

Corresponding SVG command: `L`.

The segment starts at the builder’s current position. If this is the very first command of the path (the builder therefore does not have a current position), the `line_to` command is replaced with a `move_to(to)`.

### `quadratic_bezier_to`

```rust
fn quadratic_bezier_to( &mut self, ctrl: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Adds a quadratic bézier segment to the current sub-path.

Corresponding SVG command: `Q`.

The segment starts at the builder’s current position. If this is the very first command of the path (the builder therefore does not have a current position), the `quadratic_bezier_to` command is replaced with a `move_to(to)`.

### `cubic_bezier_to`

```rust
fn cubic_bezier_to( &mut self, ctrl1: Point2D<f32, UnknownUnit>, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Adds a cubic bézier segment to the current sub-path.

Corresponding SVG command: `C`.

The segment starts at the builder’s current position. If this is the very first command of the path (the builder therefore does not have a current position), the `cubic_bezier_to` command is replaced with a `move_to(to)`.

### `relative_move_to`

```rust
fn relative_move_to(&mut self, to: Vector2D<f32, UnknownUnit>)
```

Equivalent to `move_to` in relative coordinates.

Corresponding SVG command: `m`.

The provided coordinates are offsets relative to the current position of the builder.

### `relative_line_to`

```rust
fn relative_line_to(&mut self, to: Vector2D<f32, UnknownUnit>)
```

Equivalent to `line_to` in relative coordinates.

Corresponding SVG command: `l`.

The provided coordinates are offsets relative to the current position of the builder.

### `relative_quadratic_bezier_to`

```rust
fn relative_quadratic_bezier_to( &mut self, ctrl: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `quadratic_bezier_to` in relative coordinates.

Corresponding SVG command: `q`.

the provided coordinates are offsets relative to the current position of the builder.

### `relative_cubic_bezier_to`

```rust
fn relative_cubic_bezier_to( &mut self, ctrl1: Vector2D<f32, UnknownUnit>, ctrl2: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `cubic_bezier_to` in relative coordinates.

Corresponding SVG command: `c`.

The provided coordinates are offsets relative to the current position of the builder.

### `smooth_cubic_bezier_to`

```rust
fn smooth_cubic_bezier_to( &mut self, ctrl2: Point2D<f32, UnknownUnit>, to: Point2D<f32, UnknownUnit>, )
```

Equivalent to `cubic_bezier_to` with implicit first control point.

Corresponding SVG command: `S`.

The first control point is assumed to be the reflection of the second control point on the previous command relative to the current point. If there is no previous command or if the previous command was not a cubic bézier segment, the first control point is coincident with the current position.

### `smooth_relative_cubic_bezier_to`

```rust
fn smooth_relative_cubic_bezier_to( &mut self, ctrl2: Vector2D<f32, UnknownUnit>, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `smooth_cubic_bezier_to` in relative coordinates.

Corresponding SVG command: `s`.

The provided coordinates are offsets relative to the current position of the builder.

### `smooth_quadratic_bezier_to`

```rust
fn smooth_quadratic_bezier_to(&mut self, to: Point2D<f32, UnknownUnit>)
```

Equivalent to `quadratic_bezier_to` with implicit control point.

Corresponding SVG command: `T`.

The control point is assumed to be the reflection of the control point on the previous command relative to the current point. If there is no previous command or if the previous command was not a quadratic bézier segment, a line segment is added instead.

### `smooth_relative_quadratic_bezier_to`

```rust
fn smooth_relative_quadratic_bezier_to( &mut self, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `smooth_quadratic_bezier_to` in relative coordinates.

Corresponding SVG command: `t`.

The provided coordinates are offsets relative to the current position of the builder.

### `horizontal_line_to`

```rust
fn horizontal_line_to(&mut self, x: f32)
```

Adds an horizontal line segment.

Corresponding SVG command: `H`.

Equivalent to `line_to`, using the y coordinate of the current position.

### `relative_horizontal_line_to`

```rust
fn relative_horizontal_line_to(&mut self, dx: f32)
```

Adds an horizontal line segment in relative coordinates.

Corresponding SVG command: `l`.

Equivalent to `line_to`, using the y coordinate of the current position. `dx` is the horizontal offset relative to the current position.

### `vertical_line_to`

```rust
fn vertical_line_to(&mut self, y: f32)
```

Adds a vertical line segment.

Corresponding SVG command: `V`.

Equivalent to `line_to`, using the x coordinate of the current position.

### `relative_vertical_line_to`

```rust
fn relative_vertical_line_to(&mut self, dy: f32)
```

Adds a vertical line segment in relative coordinates.

Corresponding SVG command: `v`.

Equivalent to `line_to`, using the y coordinate of the current position. `dy` is the horizontal offset relative to the current position.

### `arc_to`

```rust
fn arc_to( &mut self, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, flags: ArcFlags, to: Point2D<f32, UnknownUnit>, )
```

Adds an elliptical arc.

Corresponding SVG command: `A`.

The arc starts at the current point and ends at `to`. The size and orientation of the ellipse are defined by `radii` and an `x_rotation`, which indicates how the ellipse as a whole is rotated relative to the current coordinate system. The center of the ellipse is calculated automatically to satisfy the constraints imposed by the other parameters. the arc `flags` contribute to the automatic calculations and help determine how the arc is built.

### `relative_arc_to`

```rust
fn relative_arc_to( &mut self, radii: Vector2D<f32, UnknownUnit>, x_rotation: Angle<f32>, flags: ArcFlags, to: Vector2D<f32, UnknownUnit>, )
```

Equivalent to `arc_to` in relative coordinates.

Corresponding SVG command: `a`.

The provided `to` coordinates are offsets relative to the current position of the builder.

## Provided Methods

### `reserve`

```rust
fn reserve(&mut self, _endpoints: usize, _ctrl_points: usize)
```

Hints at the builder that a certain number of endpoints and control points will be added.

The Builder implementation may use this information to pre-allocate memory as an optimization.

### `add_polygon`

```rust
fn add_polygon(&mut self, polygon: Polygon<'_, Point2D<f32, UnknownUnit>>)
```

Adds a sub-path from a polygon.

There must be no sub-path in progress when this method is called. No sub-path is in progress after the method is called.

## Implementors

- `impl<Builder> SvgPathBuilder for WithSvg<Builder> where Builder: PathBuilder,`
