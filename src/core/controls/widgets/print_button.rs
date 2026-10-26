use iced::alignment;
use iced::widget::space;
use iced::widget::text;
use iced::widget::{button, container, row};
use iced::{Length, Shadow, Vector, Widget, border, color, font};

use crate::core::controls::models::{Message, State};
use crate::core::icons;

pub fn print_button(_state: &State) -> impl Widget<Message> {
    button(
        container(
            row![
                icons::printer().size(18).color(color!(0xffffff)),
                space().width(Length::Fixed(10.0)),
                text("Imprimir")
                    .size(15)
                    .color(color!(0xffffff))
                    .font(iced::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    }),
            ]
            .align_y(alignment::Vertical::Center),
        )
        .center_x(Length::Fill),
    )
    .width(Length::Fill)
    .padding([12, 24])
    .on_press(Message::PrintPressed)
    .style(|_theme, status| {
        let background = match status {
            button::Status::Hovered => color!(0x06967f),
            button::Status::Pressed => color!(0x057a67),
            _ => color!(0x07bc9f),
        };

        button::Style {
            background: Some(background.into()),
            text_color: color!(0xffffff),
            border: border::rounded(10),
            shadow: Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
                color: color!(0x07bc9f, 0.25),
            },
            snap: true,
        }
    })
}
