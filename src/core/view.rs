use iced::{
    Length, Widget, widget::{canvas, column, row, space, stack, text},
};

use crate::core::widgets::dot_grid::DotGrid;
use crate::core::models::Message;

pub fn view(_state: &()) -> impl Widget<Message> {
    app(_state)
}

fn app(_state: &()) -> impl Widget<Message> {
    stack![
        canvas(DotGrid::default())
            .width(Length::Fill)
            .height(Length::Fill),
        column![
            crate::core::navbar::view::view(_state).map(Message::Navbar),
            row![
                text("Left"),
                space().width(Length::Fill),
                text("Center"),
                space().width(Length::Fill),
                text("Right"),
            ],
            space().height(Length::Fill),
            row![
                space().width(Length::Fixed(10.0)),
                crate::core::controls::view::view(_state).map(Message::Controls),
                space().width(Length::Fixed(10.0)),
            ],
            space().height(Length::Fixed(10.0)),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    ]
}
