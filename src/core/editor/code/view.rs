use iced::Length;
use iced::Widget;
use iced::alignment;
use iced::border;
use iced::color;
use iced::widget::{container, row, scrollable};

use crate::core::editor::code::widgets::code_input::code_input;
use crate::core::editor::code::widgets::line_numbers::line_numbers;
use crate::core::editor::models::{Message, State};

pub fn view(state: &State) -> impl Widget<Message> + '_ {
    code(state)
}

fn code(state: &State) -> impl Widget<Message> + '_ {
    container(
        scrollable(
            row![line_numbers(state), code_input(state)]
                .width(Length::Shrink)
                .height(Length::Shrink)
                .align_y(alignment::Vertical::Top),
        )
        .smooth_scroll(false)
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::default(),
            horizontal: scrollable::Scrollbar::default(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(crate::core::widgets::dark_scrollbar_style),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0x262820))
            .border(border::rounded(8))
    })
}

