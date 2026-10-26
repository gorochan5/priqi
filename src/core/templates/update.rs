use iced::Task;

use crate::core::templates::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
    
        Message::Nothing => Task::none(),
    }
}

