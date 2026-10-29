use iced::widget::{container, responsive, row, scrollable, text, text_editor};
use iced::{Background, Border, Font, Length, Widget, alignment, border, color};

use crate::core::editor::code::widgets::highlighter::{ZplParser, zpl_highlight};
use crate::core::templates::models::{Message, State};

pub fn paste_editor(state: &State) -> impl Widget<Message> + '_ {
    let total_lines = state.paste_content.line_count().max(1);
    let numbers = (1..=total_lines)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    container(responsive(move |size| {
        let line_numbers = container(
            text(numbers.clone())
                .font(Font::MONOSPACE)
                .size(13)
                .line_height(1.5)
                .color(color!(0x7a7c74))
                .align_x(alignment::Horizontal::Right),
        )
        .padding([10, 12]);

        let input = text_editor(&state.paste_content)
            .placeholder("Cole seu código ZPL aqui...")
            .on_action(Message::CodeEditorAction)
            .font(Font::MONOSPACE)
            .size(13)
            .line_height(1.5)
            .padding(10)
            .wrapping(text::Wrapping::None)
            .height(Length::Shrink.min(size.height))
            .highlight_with::<ZplParser>((), zpl_highlight)
            .style(|_theme, _status| text_editor::Style {
                background: Background::Color(color!(0x262820)),
                border: Border::default(),
                placeholder: color!(0x7a7c74),
                value: color!(0xfffff8),
                selection: color!(0x494d3e),
            });

        scrollable(
            row![line_numbers, input]
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

