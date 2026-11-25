use iced::Length;
use iced::Widget;
use iced::alignment;
use iced::color;
use iced::padding;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::space;

use crate::core::navbar::models::{Message, State};
use crate::core::navbar::widgets;

pub fn view(state: &State) -> impl Widget<Message> {
    navbar(state)
}

fn navbar(state: &State) -> impl Widget<Message> {
    container(
        row![
            mouse_area(
                row![
                    container(widgets::branding::render(state))
                        .padding(padding::left(12))
                        .height(Length::Fill)
                        .align_y(alignment::Vertical::Center),
                    space().width(Length::Fill).height(Length::Fill),
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(alignment::Vertical::Center),
            )
            .on_press(Message::Drag)
            .on_double_click(Message::Maximize),
            row![
                widgets::minimize::render(state),
                widgets::maximize::render(state),
                widgets::exit::render(state),
            ]
            .spacing(6)
            .padding(padding::right(10))
            .height(Length::Fill)
            .align_y(alignment::Vertical::Center),
        ]
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .style(|_theme| container::background(color!(0xffffff)))
}
