use iced::widget::{column, container, row, rule, space, text};
use iced::{Length, Shadow, Vector, Widget, alignment, border, color};

use crate::core::icons::qr_code;
use crate::core::templates::models::{Message, State};
use crate::core::templates::widgets::upload_button::upload_button;

pub fn view(state: &State) -> impl Widget<Message> {
    templates(state)
}

fn templates(state: &State) -> impl Widget<Message> {
    container(
        column![
            row![
                qr_code().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                column![
                    text("").size(5),
                    text("TEMPLATES").size(14).color(color!(0x64748b)),
                ],
                space().width(Length::Fill),
                upload_button(state),
            ]
            .align_y(alignment::Vertical::Center),
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xeaeaea),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            // things here i guess
            text("Template")
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::FillPortion(20))
    .height(Length::Fill)
    .padding(5)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xfefefe))
            .border(border::rounded(10).color(color!(0xeaeaea)).width(1.5))
            .shadow(Shadow {
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
                color: color!(0x000000, 0.1),
            })
    })
}
