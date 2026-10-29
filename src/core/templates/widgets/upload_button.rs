use iced::widget::{button, container};
use iced::{Length, Shadow, Widget, border, color};

use crate::core::icons::arrow_down_to_line;
use crate::core::templates::models::{Message, State};

pub fn upload_button(_state: &State) -> impl Widget<Message> {
    button(
        container(arrow_down_to_line().size(18).color(color!(0x07bc9f)))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding(4)
    .on_press(Message::OpenUploadModal)
    .style(|_theme, status| {
        let background = match status {
            button::Status::Hovered => Some(color!(0xe8f6f3).into()),
            button::Status::Pressed => Some(color!(0xdcf2ee).into()),
            _ => None,
        };

        button::Style {
            background,
            text_color: color!(0x07bc9f),
            border: border::rounded(6),
            shadow: Shadow::default(),
            snap: true,
        }
    })
}

