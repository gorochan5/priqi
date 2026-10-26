use iced::{
    Length, Widget,
    widget::{canvas, column, row, space, stack, text},
};

use crate::core::models::{Message, State};
use crate::core::widgets::dot_grid::DotGrid;

pub fn view(state: &State) -> impl Widget<Message> {
    app(state)
}

fn app(state: &State) -> impl Widget<Message> {
    stack![
        canvas(DotGrid::default())
            .width(Length::Fill)
            .height(Length::Fill),
        column![
            crate::core::navbar::view::view(&state.navbar).map(Message::Navbar),
            space().height(Length::Fixed(10.0)),
            row![
                text("Left"),
                space().width(Length::Fill),
                crate::core::editor::view::view(&state.editor).map(Message::Editor),
                space().width(Length::Fill),
                text("Right"),
            ],
            space().height(Length::Fixed(10.0)),
            row![
                space().width(Length::Fixed(10.0)),
                crate::core::controls::view::view(&state.controls).map(Message::Controls),
                space().width(Length::Fixed(10.0)),
            ],
            space().height(Length::Fixed(10.0)),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    ]
}
