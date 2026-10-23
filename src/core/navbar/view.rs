use iced::Length;
use iced::Widget;
use iced::color;
use iced::widget::button;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::space;

use crate::core::icons;
use crate::core::navbar::models::Message;
use crate::core::navbar::widgets;

pub fn view(_state: &()) -> impl Widget<Message> {
    navbar(_state)
}

fn navbar(_state: &()) -> impl Widget<Message> {
    container(
        row![
            widgets::branding::render(_state),
            mouse_area(
                space()
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .on_press(Message::Drag),
            row![
                widgets::minimize::render(_state),
                widgets::maximize::render(_state),
                widgets::exit::render(_state),
            ]
        ]
    )
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .style(|_theme| container::background(color!(0xffffff)))
}
