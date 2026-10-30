use iced::overlay::menu;
use iced::widget::pick_list;
use iced::{Length, Shadow, Vector, Widget, border, color};

use crate::core::datasets::models::{Dataset, Message, State};

pub fn datasets_dropdown(state: &State) -> impl Widget<Message> {
    pick_list(state.dataset, &Dataset::ALL[..], Dataset::to_string)
        .placeholder("Selecione um dataset...")
        .on_select(Message::DatasetSelected)
        .width(Length::Fixed(200.0))
        .style(|theme, status| {
            let mut style = pick_list::default(theme, status);
            style.background = color!(0xffffff).into();
            style.border = border::rounded(8).color(color!(0xe2e8f0)).width(1.0);
            style.text_color = color!(0x1e293b);
            style.handle_color = color!(0x07bc9f);
            style
        })
        .menu_style(|theme| {
            let mut style = menu::default(theme);
            style.background = color!(0xffffff).into();
            style.border = border::rounded(0).color(color!(0xe2e8f0)).width(1.0);
            style.text_color = color!(0x1e293b);
            style.selected_background = color!(0x07bc9f).into();
            style.selected_text_color = color!(0xffffff);
            style.shadow = Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
                color: color!(0x000000, 0.08),
            };
            style
        })
}
