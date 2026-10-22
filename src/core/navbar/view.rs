use iced::Widget
use iced::widget::button;
use iced::widget::container
use iced::widget::text;
use iced::widget::row;

use crate::core::icons;
use crate::core::models::Message;

pub fn view(_state: &()) -> impl Widget<Message> {
    navbar(_state)
}

fn navbar(_state: &()) -> impl Widget<Message> {
    let minimize = button(icons::minus()).on_press(Message::Minimize)

    container(
        row![

        ]
    )
}
