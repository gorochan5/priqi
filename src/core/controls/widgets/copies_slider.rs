use iced::alignment;
use iced::widget::column;
use iced::widget::slider::{Handle, HandleShape, Rail};
use iced::widget::space;
use iced::widget::text;
use iced::widget::{container, row, slider};
use iced::{Length, Widget, border, color};

use crate::core::controls::models::{Message, State};
use crate::core::icons;

pub fn copies_slider(state: &State) -> impl Widget<Message> {
    let copies: u32 = state.copies;
    let min = 1.0_f32;
    let max = 100.0_f32;
    let slider_width = 180.0_f32;
    let badge_width = 32.0_f32;
    let handle_radius = 9.0_f32;

    let percent = ((copies as f32 - min) / (max - min)).clamp(0.0, 1.0);
    let usable_width = slider_width - (handle_radius * 2.0);
    let handle_center_x = handle_radius + (usable_width * percent);
    let badge_offset =
        (handle_center_x - (badge_width / 2.0)).clamp(0.0, slider_width - badge_width);

    let copies_slider = slider(1..=100, copies, Message::CopiesChanged)
        .step(1)
        .default(1u32)
        .shift_step(10)
        .width(Length::Fixed(slider_width))
        .height(20.0)
        .style(move |_theme, status| {
            let (active_color, handle_bg) = match status {
                slider::Status::Hovered => (color!(0x06967f), color!(0xf0fdfa)),
                slider::Status::Dragged => (color!(0x057a67), color!(0xe6f4f1)),
                slider::Status::Active => (color!(0x07bc9f), color!(0xffffff)),
            };
            slider::Style {
                rail: Rail {
                    backgrounds: (active_color.into(), color!(0xdbe7e5).into()),
                    width: 6.0,
                    border: border::rounded(3.0),
                },
                handle: Handle {
                    shape: HandleShape::Circle {
                        radius: handle_radius,
                    },
                    background: handle_bg.into(),
                    border_width: 4.0,
                    border_color: active_color,
                },
            }
        });

    let copies_badge = container(text(format!("{copies}")).size(11).color(color!(0xffffff)))
        .width(Length::Fixed(badge_width))
        .padding([2, 6])
        .align_x(alignment::Horizontal::Center)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0x07bc9f))
                .border(border::rounded(6))
        });

    column![
        column![
            copies_slider,
            space().height(Length::Fixed(2.0)),
            row![space().width(Length::Fixed(badge_offset)), copies_badge],
        ],
        space().height(Length::Fixed(6.0)),
        row![
            icons::copy().size(20).color(color!(0x07bc9f)),
            space().width(Length::Fixed(6.0)),
            text("CÓPIAS").size(13).color(color!(0x64748b)),
        ]
        .align_y(alignment::Vertical::Center),
    ]
}
