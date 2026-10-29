use iced::widget::canvas::{self, Geometry, LineDash, Path, Stroke};
use iced::widget::{button, center, column, container, space, text};
use iced::{
    Background, Color, Font, Length, Point, Rectangle, Renderer, Size, Theme, Widget, alignment,
    border, color, font,
};

use crate::core::icons::cloud_upload;
use crate::core::templates::models::{Message, State};

pub fn dropzone(_state: &State) -> impl Widget<Message> {
    button(
        container(
            iced::widget::stack![
                canvas::Canvas::new(DashedBorderCanvas::default())
                    .width(Length::Fill)
                    .height(Length::Fill),
                center(
                    column![
                        cloud_upload().size(48).color(color!(0x64748b)),
                        space().height(Length::Fixed(12.0)),
                        text("Clique para selecionar ou arraste um arquivo até aqui")
                            .size(14)
                            .font(Font {
                                weight: font::Weight::Medium,
                                ..Default::default()
                            })
                            .color(color!(0x334155)),
                        space().height(Length::Fixed(4.0)),
                        text("Suporta arquivos de etiqueta (.zpl, .txt, .prn)")
                            .size(12)
                            .color(color!(0x94a3b8)),
                    ]
                    .align_x(alignment::Horizontal::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill),
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .on_press(Message::OpenFilePicker)
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(0)
    .style(|_theme, status| {
        let bg = match status {
            button::Status::Hovered => color!(0xf1f5f9),
            button::Status::Pressed => color!(0xe2e8f0),
            _ => color!(0xf8fafc),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            border: border::rounded(12),
            ..Default::default()
        }
    })
}

#[derive(Debug, Clone, Copy)]
struct DashedBorderCanvas {
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
}

impl Default for DashedBorderCanvas {
    fn default() -> Self {
        Self {
            border_color: color!(0x94a3b8),
            border_width: 2.0,
            radius: 12.0,
        }
    }
}

impl<Message> canvas::Program<Message> for DashedBorderCanvas {
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
            let offset = self.border_width / 2.0;
            let width = (bounds.width - self.border_width).max(0.0);
            let height = (bounds.height - self.border_width).max(0.0);

            let path = Path::rounded_rectangle(
                Point::new(offset, offset),
                Size::new(width, height),
                border::radius(self.radius),
            );

            let stroke = Stroke {
                style: canvas::stroke::Style::Solid(self.border_color),
                width: self.border_width,
                line_dash: LineDash {
                    segments: &[8.0, 6.0],
                    offset: 0,
                },
                line_cap: canvas::LineCap::Round,
                line_join: canvas::LineJoin::Round,
            };

            frame.stroke(&path, stroke);
        });

        vec![geometry]
    }
}

