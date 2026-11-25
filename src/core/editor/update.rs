use iced::Task;

use crate::core::editor::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectTab(tab) => {
            if state.active_tab == tab {
                return Task::none();
            }
            state.active_tab = tab;
            state.is_focused = false;
            Task::none()
        }
        Message::Edit(action) => {
            state.is_focused = true;
            if action.is_edit() {
                state.is_dirty = true;
            }
            state.content.perform(action);
            state.sync_line_numbers();
            Task::none()
        }
        Message::Unfocus => {
            state.is_focused = false;
            Task::none()
        }
        Message::RotateLeft => {
            state.rotation = (state.rotation + 270) % 360;
            Task::none()
        }
        Message::RotateRight => {
            state.rotation = (state.rotation + 90) % 360;
            Task::none()
        }
        Message::LabelWidthChanged(val) => {
            state.width_input = val.clone();
            if let Ok(parsed) = val.trim().replace(',', ".").parse::<f32>()
                && (10.0..=500.0).contains(&parsed)
            {
                state.label_width_mm = parsed;
            }
            Task::none()
        }
        Message::LabelHeightChanged(val) => {
            state.height_input = val.clone();
            if let Ok(parsed) = val.trim().replace(',', ".").parse::<f32>()
                && (10.0..=500.0).contains(&parsed)
            {
                state.label_height_mm = parsed;
            }
            Task::none()
        }
        Message::LabelDpiSelected(dpi) => {
            state.label_dpi = dpi;
            Task::none()
        }
        Message::MaximizeCard(idx) => {
            state.maximized_card = Some(idx);
            Task::none()
        }
        Message::UnmaximizeCard => {
            state.maximized_card = None;
            Task::none()
        }
        Message::PrevMaximizedCard => {
            if let Some(curr) = state.maximized_card
                && curr > 0
            {
                state.maximized_card = Some(curr - 1);
            }
            Task::none()
        }
        Message::NextMaximizedCard => {
            if let Some(curr) = state.maximized_card
                && curr + 1 < state.preview_cards.len()
            {
                state.maximized_card = Some(curr + 1);
            }
            Task::none()
        }
    }
}
