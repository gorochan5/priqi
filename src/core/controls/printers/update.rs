use iced::Task;

use crate::core::controls::printers::models::Message;

pub fn update(_state: &mut (), message: Message) -> Task<Message> {
    match message {
        Message::PrinterSelected(_printer) => Task::none(),
        Message::Nothing => Task::none(),
    }
}
