use iced::alignment;
use iced::widget::space;
use iced::widget::text;
use iced::widget::{button, container, row};
use iced::{Length, Shadow, Vector, Widget, border, color, font};

use crate::core::controls::models::{Message, State};
use crate::core::icons;

pub fn print_button(state: &State) -> impl Widget<Message> {
    let label = if state.is_printing {
        "Imprimindo..."
    } else {
        "Imprimir"
    };
    let on_press = (state.can_print && !state.is_printing && state.printer.is_some())
        .then_some(Message::PrintPressed);

    button(
        container(
            row![
                icons::printer().size(18).color(color!(0xffffff)),
                space().width(Length::Fixed(10.0)),
                text(label)
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
    .on_press_maybe(on_press)
    .style(|_theme, status| {
        let (background, shadow_color) = match status {
            button::Status::Disabled => (color!(0x94a3b8), color!(0x000000, 0.0)),
            button::Status::Hovered => (color!(0x06967f), color!(0x07bc9f, 0.25)),
            button::Status::Pressed => (color!(0x057a67), color!(0x07bc9f, 0.25)),
            _ => (color!(0x07bc9f), color!(0x07bc9f, 0.25)),
        };

        button::Style {
            background: Some(background.into()),
            text_color: color!(0xffffff),
            border: border::rounded(10),
            shadow: Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
                color: shadow_color,
            },
            snap: true,
        }
    })
}
