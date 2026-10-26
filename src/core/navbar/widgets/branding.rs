use iced::{
    Length, Widget,
    widget::{Button, Image, button, column, row, space},
};

use crate::core::{
    icons,
    navbar::{models::Message, widgets::version},
};

pub fn render<'a>(s_state: &()) -> impl Widget<Message> + 'a {
    let version = version::render(s_state);
    column![
        space().height(8.0),
        row![
            icons::branding()
                .width(Length::Fixed(128.0))
                .height(Length::Fixed(32.0)),
            column![space().height(8.0), version,]
        ]
    ]
}
