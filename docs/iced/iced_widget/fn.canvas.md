---
title: "canvas"
kind: function
path: "iced_widget::canvas"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.canvas.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.canvas.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1602-1608"
---

# `iced_widget::canvas`

```rust
pub fn canvas<P, Message, Theme, Renderer>(
    program: P,
) -> Canvas<P, Message, Theme, Renderer> where
    Renderer: Renderer,
    P: Program<Message, Theme, Renderer>,
```

Creates a new [`Canvas`](canvas/struct.Canvas.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1602-1608)
