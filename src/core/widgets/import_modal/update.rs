use iced::Task;

use super::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Open => {
            state.is_open = true;
            Task::none()
        }
        Message::Close => {
            state.is_open = false;
            Task::none()
        }
        Message::SelectTab(tab) => {
            state.active_tab = tab;
            Task::none()
        }
        Message::EditorAction(action) => {
            state.content.perform(action);
            Task::none()
        }
        Message::OpenFilePicker => {
            // Por enquanto não faz nada
            Task::none()
        }
        Message::ImportPressed => {
            // Por enquanto não faz nada
            Task::none()
        }
    }
}

