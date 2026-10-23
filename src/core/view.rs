use iced::{
    Widget, color,
    widget::{column, container, row, text},
};

use crate::core::models::Message;


pub fn view(_state: &()) -> impl Widget<Message> {
    app(_state)
}

fn app(_state: &()) -> impl Widget<Message> {
    container(
        column![
            crate::core::navbar::view::view(_state).map(Message::Navbar),
            row![
                text("Left"),
                text("Center"),
                text("Right"),
            ],
            "Bottom",
        ]
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .style(|_theme| container::background(color!(0xf4f6f5)))
}