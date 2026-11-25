use iced::{
    Length, Widget, alignment,
    widget::{row, space},
};

use crate::core::{
    icons,
    navbar::{
        models::{Message, State},
        widgets::version,
    },
};

pub fn render<'a>(state: &State) -> impl Widget<Message> + 'a {
    let version = version::render(state);
    row![
        icons::branding()
            .width(Length::Fixed(128.0))
            .height(Length::Fixed(32.0)),
        space().width(Length::Fixed(6.0)),
        version,
    ]
    .align_y(alignment::Vertical::Center)
}
