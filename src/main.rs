use crate::core::{update::update, view::view};


mod core;

pub fn main() -> iced::Result {
    iced::application(<()>::default, update, view)
        .window(iced::window::Settings {
            maximized: true,
            resizable: true,
            ..Default::default()
        })
        .fonts([core::icons::FONT_BYTES])
        .centered()
        .title("Hello World")
        .run()
}

