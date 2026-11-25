use iced::widget::canvas::{self, Geometry, LineDash, Path, Stroke};
use iced::widget::{button, center, column, container, space, text};
use iced::{
    Background, Color, Element, Font, Length, Point, Rectangle, Renderer, Size, Theme, Widget,
    alignment, border, color, font,
};

use crate::core::icons::{circle_alert, cloud_upload, file_check};
use crate::core::widgets::import_modal::models::{ImportConfig, Message, State};

pub fn dropzone<'a>(state: &'a State, config: ImportConfig) -> impl Widget<Message> + 'a {
    let border_color = if state.file_error.is_some() {
        color!(0xef4444)
    } else if state.is_hovered || state.selected_file.is_some() {
        color!(0x07bc9f)
    } else {
        color!(0x94a3b8)
    };

    let inner_content: Element<'a, Message> = if let Some(err) = &state.file_error {
        column![
            circle_alert().size(46).color(color!(0xef4444)),
            space().height(Length::Fixed(12.0)),
            text(err)
                .size(14)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Default::default()
                })
                .color(color!(0xdc2626)),
            space().height(Length::Fixed(6.0)),
            text("Clique para tentar selecionar outro arquivo ou arraste-o até aqui")
                .size(12)
                .color(color!(0x64748b)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    } else if let Some(path) = &state.selected_file {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Arquivo selecionado")
            .to_string();
        let line_count = state.content.line_count();

        column![
            file_check().size(48).color(color!(0x07bc9f)),
            space().height(Length::Fixed(12.0)),
            text(file_name)
                .size(15)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x0f172a)),
            space().height(Length::Fixed(4.0)),
            text(format!(
                "Arquivo carregado ({line_count} linhas) — clique em \"Importar\" abaixo ou clique aqui para trocar"
            ))
            .size(12)
            .color(color!(0x06967f)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    } else {
        let icon_color = if state.is_hovered {
            color!(0x07bc9f)
        } else {
            color!(0x64748b)
        };
        let prompt_text = if state.is_hovered {
            "Solte o arquivo aqui para carregar"
        } else {
            "Clique para selecionar ou arraste um arquivo até aqui"
        };

        column![
            cloud_upload().size(48).color(icon_color),
            space().height(Length::Fixed(12.0)),
            text(prompt_text)
                .size(14)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Default::default()
                })
                .color(color!(0x334155)),
            space().height(Length::Fixed(4.0)),
            text(config.dropzone_hint)
                .size(12)
                .color(color!(0x94a3b8)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    };

    let is_hovered = state.is_hovered;
    let has_file = state.selected_file.is_some();

    button(
        container(
            iced::widget::stack![
                canvas::Canvas::new(DashedBorderCanvas {
                    border_color,
                    ..Default::default()
                })
                .width(Length::Fill)
                .height(Length::Fill),
                center(inner_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(16),
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .on_press(Message::OpenFilePicker(config.kind))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(0)
    .style(move |_theme, status| {
        let bg = if is_hovered {
            color!(0xe8f6f3)
        } else if has_file {
            match status {
                button::Status::Hovered => color!(0xe8f6f3),
                button::Status::Pressed => color!(0xdcf2ee),
                _ => color!(0xf0fdf9),
            }
        } else {
            match status {
                button::Status::Hovered => color!(0xf1f5f9),
                button::Status::Pressed => color!(0xe2e8f0),
                _ => color!(0xf8fafc),
            }
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
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
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

        vec![frame.into_geometry()]
    }
}
