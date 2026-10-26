use iced::Font;
use iced::Widget;
use iced::alignment;
use iced::color;
use iced::widget::{container, text};

use crate::core::editor::models::{Message, State};

pub fn line_numbers(state: &State) -> impl Widget<Message> {
    let total_lines = state.content.line_count().max(1);
    let numbers = (1..=total_lines)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    container(
        text(numbers)
            .font(Font::MONOSPACE)
            .size(14)
            .line_height(1.5)
            .color(color!(0x7a7c74))
            .align_x(alignment::Horizontal::Right),
    )
    .padding([10, 12])
}

