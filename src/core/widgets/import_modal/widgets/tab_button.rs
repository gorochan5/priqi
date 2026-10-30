use iced::widget::{button, container, text};
use iced::{Background, Color, Font, Shadow, Vector, Widget, border, color, font};

use crate::core::widgets::import_modal::models::{ImportTab, Message};

pub fn tab_button<'a>(
    label: &'a str,
    tab: ImportTab,
    is_active: bool,
    is_left: bool,
) -> impl Widget<Message> + 'a {
    let text_color = if is_active {
        color!(0x0f172a)
    } else {
        color!(0x64748b)
    };

    let font_weight = if is_active {
        font::Weight::Bold
    } else {
        font::Weight::Medium
    };

    let border_radius = if is_left {
        border::rounded(border::left(16))
    } else {
        border::rounded(border::right(16))
    };

    button(
        container(
            text(label)
                .size(13)
                .color(text_color)
                .font(Font {
                    weight: font_weight,
                    ..Default::default()
                }),
        )
        .padding([6, 18]),
    )
    .on_press(Message::SelectTab(tab))
    .style(move |_theme, status| {
        let bg = if is_active {
            color!(0xffffff)
        } else {
            match status {
                button::Status::Hovered => color!(0xf1f5f9),
                _ => Color::TRANSPARENT,
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: border_radius,
            shadow: if is_active {
                Shadow {
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                    color: color!(0x000000, 0.08),
                }
            } else {
                Shadow::default()
            },
            snap: true,
        }
    })
}

