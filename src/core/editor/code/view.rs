use iced::Length;
use iced::Widget;
use iced::alignment;
use iced::border;
use iced::color;
use iced::widget::{container, responsive, row, scrollable};

use crate::core::editor::code::widgets::code_input::code_input;
use crate::core::editor::code::widgets::line_numbers::line_numbers;
use crate::core::editor::models::{Message, State};

pub fn view(state: &State) -> impl Widget<Message> + '_ {
    code(state)
}

fn code(state: &State) -> impl Widget<Message> + '_ {
    container(responsive(move |size| {
        scrollable(
            row![
                line_numbers(state),
                code_input(state, size.height)
            ]
            .width(Length::Fill)
            .height(Length::Shrink.min(size.height))
            .align_y(alignment::Vertical::Top),
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }))
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0x262820))
            .border(border::rounded(8))
    })
}

