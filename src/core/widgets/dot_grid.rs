use iced::widget::canvas::{self, Geometry};
use iced::{Color, Point, Rectangle, Renderer, Theme, color};

#[derive(Debug, Clone, Copy)]
pub struct DotGrid {
    pub spacing: f32,
    pub radius: f32,
    pub dot_color: Color,
    pub bg_color: Color,
}

impl Default for DotGrid {
    fn default() -> Self {
        Self {
            spacing: 24.0,
            radius: 1.0,
            dot_color: color!(0xc8d1cf),
            bg_color: color!(0xf4f6f5),
        }
    }
}

impl<Message> canvas::Program<Message> for DotGrid {
    type State = canvas::Cache;

    fn draw(
        &self,
        cache: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = cache.draw(renderer, bounds.size(), |frame| {
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), self.bg_color);

            let mut builder = canvas::path::Builder::new();
            let mut y = self.spacing / 2.0;

            while y < bounds.height {
                let mut x = self.spacing / 2.0;
                while x < bounds.width {
                    builder.circle(Point::new(x, y), self.radius);
                    x += self.spacing;
                }
                y += self.spacing;
            }

            frame.fill(&builder.build(), self.dot_color);
        });

        vec![geometry]
    }
}
