use iced::Widget;

use crate::core::editor::models::{Message, State};
use crate::core::editor::preview::widgets::preview_frame::preview_frame;

pub fn view(state: &State) -> impl Widget<Message> {
    preview(state)
}

fn preview(state: &State) -> impl Widget<Message> {
    preview_frame(state)
}

