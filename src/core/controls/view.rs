use iced::widget::{bottom_center, column, container, opaque, row};
use iced::{Length, Widget, alignment, border, color};

use crate::core::controls::models::{Message, State};
use crate::core::controls::widgets::copies_slider::copies_slider;
use crate::core::controls::widgets::print_button::print_button;
use crate::core::controls::widgets::printers_dropdown::printers_dropdown;
use crate::core::controls::widgets::toggle_button::toggle_button;

pub fn view(state: &State) -> impl Widget<Message> {
    controls(state)
}

fn controls(state: &State) -> impl Widget<Message> {
    let mut content = column![toggle_button(state)]
        .align_x(alignment::Horizontal::Center)
        .width(Length::Shrink);

    if !state.collapsed {
        let card_column = column![
            row![printers_dropdown(state), copies_slider(state)]
                .spacing(64)
                .align_y(alignment::Vertical::Bottom),
            print_button(state),
        ]
        .spacing(10)
        .width(Length::Shrink);

        let card = container(card_column)
            .padding(10)
            .width(Length::Shrink)
            .style(|_theme| {
                container::Style::default()
                    .background(color!(0xfefefe))
                    .border(border::rounded(10).color(color!(0xd8dfdd)).width(1.0))
            });

        content = content.push(opaque(card).boxed());
    }

    bottom_center(content)
}
