use crate::core::{
    icons,
    navbar::models::{Message, State},
};
use iced::{
    Length, Widget, border, color,
    widget::{button, container},
};

pub fn render<'a>(state: &State) -> impl Widget<Message> + 'a {
    let icon = if state.is_maximized {
        icons::copy()
    } else {
        icons::square()
    };

    button(
        container(icon.size(15))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .on_press(Message::Maximize)
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0))
    .padding(0)
    .style(|_theme, status| {
        let (bg, border_col, icon_col) = match status {
            button::Status::Hovered => (color!(0xe8f6f3), color!(0xb6e3db), color!(0x07bc9f)),
            button::Status::Pressed => (color!(0xdcf2ee), color!(0xb6e3db), color!(0x07bc9f)),
            _ => (color!(0xf8fafc), color!(0xe2e8f0), color!(0x475569)),
        };

        button::Style {
            background: Some(bg.into()),
            text_color: icon_col,
            border: border::rounded(8).color(border_col).width(1.0),
            ..button::Style::default()
        }
    })
}
