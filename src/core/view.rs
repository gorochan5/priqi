use iced::{Widget, widget::{column, container, row, text}};

use crate::core::models::Message;


pub fn view(_state: &()) -> impl Widget<Message> {
    app(_state)
}

fn app(_state: &()) -> impl Widget<Message> {
    container(
        column![
            crate::core::navbar::view::view(_state),
            row![
                text("Left"),
                text("Center"),
                text("Right"),
            ],
            "Bottom",
        ]
    )
}