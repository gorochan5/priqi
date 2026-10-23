use iced::{Length, Widget, color, widget::button};
use crate::core::{icons, navbar::models::Message};

pub fn render(_state: &()) -> impl Widget<Message> {
    button(icons::x().size(24))
        .on_press(Message::Close)
        .height(Length::Fill)
        .style(|_theme, status| match status {
            button::Status::Hovered => button::Style {
                background: Some(color!(0xF3FEFD).into()),
                text_color: color!(0x080808),
                ..button::Style::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(color!(0xDAFDF8).into()),
                text_color: color!(0x080808),
                ..button::Style::default()
            },
            _ => button::Style {
                background: None,
                ..button::Style::default()
            },
        })
}