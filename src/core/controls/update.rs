use iced::{Task, window};

use crate::core::controls::models::Message;

pub fn update(state: &mut (), message: Message) -> Task<Message>{
    match message {
        Message::Printers(msg) => crate::core::controls::printers::update::update(state, msg).map(Message::Printers),
        Message::Nothing => Task::none(),
    }
}
