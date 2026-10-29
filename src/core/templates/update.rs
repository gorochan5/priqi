use iced::Task;

use crate::core::templates::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::OpenUploadModal => {
            state.show_upload_modal = true;
            Task::none()
        }
        Message::CloseUploadModal => {
            state.show_upload_modal = false;
            Task::none()
        }
        Message::SelectUploadTab(tab) => {
            state.upload_tab = tab;
            Task::none()
        }
        Message::CodeEditorAction(action) => {
            state.paste_content.perform(action);
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
        Message::Nothing => Task::none(),
    }
}
