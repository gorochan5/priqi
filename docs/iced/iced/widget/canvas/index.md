---
title: "canvas"
kind: module
path: "iced::widget::canvas"
crate: "iced"
module: "iced::widget"
file: "iced/widget/canvas/index.md"
docs_url: "https://docs.iced.rs/iced/widget/canvas/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "142"
---

# `iced::widget::canvas`

Canvases can be leveraged to draw interactive 2D graphics.

## Example: Drawing a Simple Circle

```rust
use iced::mouse;
use iced::widget::canvas;
use iced::{Color, Rectangle, Renderer, Theme};

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

// Finally, we simply use our `Circle` to create the `Canvas`!
fn view<Message>(_state: &State) -> impl Widget<Message> {
    canvas(Circle { radius: 50.0 })
}
```

## Modules

- [`fill`](fill/index.md) — Fill `Geometry` with a certain style.
- [`gradient`](gradient/index.md) — A gradient that can be used as a fill for some geometry.
- [`path`](path/index.md) — Build different kinds of 2D shapes.
- [`stroke`](stroke/index.md) — Create lines from a [`Path`](struct.Path.md) and assigns them various attributes/styles.

## Structs

- [`Action`](struct.Action.md) — A runtime action that can be performed by some widgets.
- [`Canvas`](struct.Canvas.md) — A widget capable of drawing 2D graphics.
- [`Fill`](struct.Fill.md) — The style used to fill geometry.
- [`Group`](struct.Group.md) — A cache group.
- [`Image`](struct.Image.md) — A raster image that can be drawn.
- [`LineDash`](struct.LineDash.md) — The dash pattern used when stroking the line.
- [`Path`](struct.Path.md) — An immutable set of points that may or may not be connected.
- [`Stroke`](struct.Stroke.md) — The style of a stroke.
- [`Text`](struct.Text.md) — A bunch of text that can be drawn to a canvas

## Enums

- [`Event`](enum.Event.md) — A user interface event.
- [`Gradient`](enum.Gradient.md) — A fill which linearly interpolates colors along a direction.
- [`LineCap`](enum.LineCap.md) — The shape used at the end of open subpaths when they are stroked.
- [`LineJoin`](enum.LineJoin.md) — The shape used at the corners of paths or basic shapes when they are stroked.
- [`Style`](enum.Style.md) — The coloring style of some drawing.

## Traits

- [`Program`](trait.Program.md) — The state and logic of a [`Canvas`](../struct.Canvas.md).

## Type Aliases

- [`Cache`](type.Cache.md) — A simple cache that stores generated [`Geometry`](type.Geometry.md) to avoid recomputation.
- [`Frame`](type.Frame.md) — The frame supported by a renderer.
- [`Geometry`](type.Geometry.md) — The geometry supported by a renderer.
