use iced::Background;
use iced::Border;
use iced::Font;
use iced::Length;
use iced::Widget;
use iced::color;
use iced::widget::{text, text_editor};

use crate::core::editor::code::widgets::highlighter::{ZplParser, zpl_highlight};
use crate::core::editor::models::{Message, State};

pub fn code_input(state: &State) -> impl Widget<Message> + '_ {
    text_editor(&state.content)
        .placeholder("Digite o código ZPL aqui...")
        .on_action(Message::Edit)
        .key_binding(|key_press| {
            match text_editor::Binding::from_key_press(key_press) {
                Some(text_editor::Binding::Unfocus) => Some(text_editor::Binding::Sequence(vec![
                    text_editor::Binding::Unfocus,
                    text_editor::Binding::Custom(Message::Unfocus),
                ])),
                other => other,
            }
        })
        .font(Font::MONOSPACE)
        .size(14)
        .line_height(1.5)
        .padding(10)
        .wrapping(text::Wrapping::None)
        .height(Length::Shrink)
        .highlight_with::<ZplParser>((), zpl_highlight)
        .style(|_theme, _status| text_editor::Style {
            background: Background::Color(color!(0x262820)),
            border: Border::default(),
            placeholder: color!(0x7a7c74),
            value: color!(0xfffff8),
            selection: color!(0x494d3e),
        })
}
