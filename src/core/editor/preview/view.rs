use iced::Element;

use crate::core::editor::models::{Message, State};
use crate::core::editor::preview::widgets::preview_frame::preview_frame;

pub fn view(state: &State) -> Element<'_, Message> {
    preview(state)
}

fn preview(state: &State) -> Element<'_, Message> {
    preview_frame(state)
}
