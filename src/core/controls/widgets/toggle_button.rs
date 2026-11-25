use iced::widget::{button, container};
use iced::{Length, Padding, Shadow, Vector, Widget, border, color};

use crate::core::controls::models::{Message, State};
use crate::core::icons;

pub fn toggle_button(state: &State) -> impl Widget<Message> {
    let icon = if state.collapsed {
        icons::chevron_up()
    } else {
        icons::chevron_down()
    };

    let btn = button(container(icon.size(16).color(color!(0x07bc9f))).center_x(Length::Fill))
        .width(Length::Fixed(45.0))
        .padding([4, 10])
        .on_press(Message::ToggleCollapse)
        .style(|_theme, status| {
            let background = match status {
                button::Status::Hovered => color!(0xf1f5f9),
                button::Status::Pressed => color!(0xe2e8f0),
                _ => color!(0xfefefe),
            };

            button::Style {
                background: Some(background.into()),
                text_color: color!(0x07bc9f),
                border: border::rounded(border::top(6.5)),
                shadow: Shadow::default(),
                snap: true,
            }
        });

    container(btn)
        .padding(Padding {
            top: 1.5,
            right: 1.5,
            bottom: 0.0,
            left: 1.5,
        })
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xeaeaea))
                .border(border::rounded(border::top(8)))
                .shadow(Shadow {
                    offset: Vector::new(0.0, -1.0),
                    blur_radius: 4.0,
                    color: color!(0x000000, 0.06),
                })
        })
}
