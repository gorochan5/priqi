use iced::Task;

use crate::core::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Navbar(msg) => {
            crate::core::navbar::update::update(&mut state.navbar, msg).map(Message::Navbar)
        }
        Message::Controls(msg) => {
            crate::core::controls::update::update(&mut state.controls, msg).map(Message::Controls)
        }
        Message::Editor(msg) => {
            crate::core::editor::update::update(&mut state.editor, msg).map(Message::Editor)
        }
        Message::Templates(msg) => {
            crate::core::templates::update::update(&mut state.templates, msg).map(Message::Templates)
        }
        Message::Datasets(msg) => {
            crate::core::datasets::update::update(&mut state.datasets, msg).map(Message::Datasets)
        }
        Message::Nothing => Task::none(),
    }
}
