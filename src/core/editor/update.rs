use iced::Task;

use crate::core::editor::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectTab(tab) => {
            state.active_tab = tab;
            Task::none()
        }
        Message::Edit(action) => {
            state.content.perform(action);
            Task::none()
        }
    }
}

