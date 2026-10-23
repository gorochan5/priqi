use iced::Task;

use crate::core::models::Message;

pub fn update(state: &mut (), message: Message) -> Task<Message>{
    match message {
        Message::Navbar(msg) => crate::core::navbar::update::update(state, msg).map(Message::Navbar),
        Message::Nothing => Task::none(),
    }
}
