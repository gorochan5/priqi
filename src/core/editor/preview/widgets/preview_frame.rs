use iced::Length;
use iced::Widget;
use iced::border;
use iced::color;
use iced::widget::{container, text};

use crate::core::editor::models::{Message, State};

pub fn preview_frame(_state: &State) -> impl Widget<Message> {
    container(
        text("Aqui será o preview do zpl")
            .size(15)
            .color(color!(0x64748b)),
    )
    .center(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xf8fafc))
            .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
    })
}

