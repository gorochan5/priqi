use iced::Task;

use crate::core::controls::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PrinterSelected(printer) => {
            state.printer = Some(printer);
            Task::none()
        }
        Message::CopiesChanged(copies) => {
            state.copies = copies;
            Task::none()
        }
        Message::PrintPressed => Task::none(),
        Message::ToggleCollapse => {
            state.collapsed = !state.collapsed;
            Task::none()
        }
        Message::Nothing => Task::none(),
    }
}
