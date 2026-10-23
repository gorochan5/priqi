use iced::Length;
use iced::Shadow;
use iced::Vector;
use iced::Widget;
use iced::border;
use iced::color;
use iced::widget::container;

use crate::core::controls::models::Message;



pub fn view(_state: &()) -> impl Widget<Message> {
    controls(_state)
}

fn controls(_state: &()) -> impl Widget<Message> {
    container(
        crate::core::controls::printers::view::view(_state).map(Message::Printers),
    )
    .width(Length::Fill)
    .height(Length::Fixed(96.0))
    .padding(5)
    .style(
        |_theme| {
            container::Style::default()
            .background(color!(0xfefefe))
            .border(
                border::rounded(10)
                .color(color!(0xeaeaea))
                .width(1.5),
            )
            .shadow(Shadow {
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
                color: color!(0x000000, 0.1),
            })
        }
    )
}
