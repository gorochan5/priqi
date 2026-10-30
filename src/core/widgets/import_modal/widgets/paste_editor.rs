use iced::widget::{container, responsive, row, scrollable, text, text_editor};
use iced::{Background, Border, Element, Font, Length, Widget, alignment, border, color};

use super::highlighters::{CsvParser, ZplParser, csv_highlight, zpl_highlight};
use crate::core::widgets::import_modal::models::{ImportConfig, ImportKind, Message, State};

pub fn paste_editor<'a>(state: &'a State, config: ImportConfig) -> impl Widget<Message> + 'a {
    let total_lines = state.content.line_count().max(1);
    let numbers = (1..=total_lines)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let placeholder = config.placeholder;
    let kind = config.kind;

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

        let base_editor = text_editor(&state.content)
            .placeholder(placeholder)
            .on_action(Message::EditorAction)
            .font(Font::MONOSPACE)
            .size(13)
            .line_height(1.5)
            .padding(10)
            .wrapping(text::Wrapping::None)
            .height(Length::Shrink.min(size.height))
            .style(|_theme, _status| text_editor::Style {
                background: Background::Color(color!(0x262820)),
                border: Border::default(),
                placeholder: color!(0x7a7c74),
                value: color!(0xfffff8),
                selection: color!(0x494d3e),
            });

        let input: Element<'_, Message> = match kind {
            ImportKind::Templates => {
                base_editor.highlight_with::<ZplParser>((), zpl_highlight).boxed()
            }
            ImportKind::Datasets => {
                base_editor.highlight_with::<CsvParser>((), csv_highlight).boxed()
            }
        };

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
