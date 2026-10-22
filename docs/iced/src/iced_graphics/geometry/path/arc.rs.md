---
title: "src/iced_graphics/geometry/path/arc.rs"
kind: source
path: "src/iced_graphics/geometry/path/arc.rs"
crate: "iced_graphics"
file: "src/iced_graphics/geometry/path/arc.rs.md"
docs_url: "https://docs.iced.rs/src/iced_graphics/geometry/path/arc.rs.html"
source_file: "src/iced_graphics/geometry/path/arc.rs"
---

# `src/iced_graphics/geometry/path/arc.rs`

```rust
//! Build and draw curves.
use iced_core::{Point, Radians, Vector};

/// A segment of a differentiable curve.
#[derive(Debug, Clone, Copy)]
pub struct Arc {
    /// The center of the arc.
    pub center: Point,
    /// The radius of the arc.
    pub radius: f32,
    /// The start of the segment's angle, clockwise rotation from positive x-axis.
    pub start_angle: Radians,
    /// The end of the segment's angle, clockwise rotation from positive x-axis.
    pub end_angle: Radians,
}

/// An elliptical [`Arc`].
#[derive(Debug, Clone, Copy)]
pub struct Elliptical {
    /// The center of the arc.
    pub center: Point,
    /// The radii of the arc's ellipse. The horizontal and vertical half-dimensions of the ellipse will match the x and y values of the radii vector.
    pub radii: Vector,
    /// The clockwise rotation of the arc's ellipse.
    pub rotation: Radians,
    /// The start of the segment's angle, clockwise rotation from positive x-axis.
    pub start_angle: Radians,
    /// The end of the segment's angle, clockwise rotation from positive x-axis.
    pub end_angle: Radians,
}

impl From<Arc> for Elliptical {
    fn from(arc: Arc) -> Elliptical {
        Elliptical {
            center: arc.center,
            radii: Vector::new(arc.radius, arc.radius),
            rotation: Radians(0.0),
            start_angle: arc.start_angle,
            end_angle: arc.end_angle,
        }
    }
}
```
