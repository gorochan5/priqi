use iced::{Task, window};

use crate::core::navbar::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Minimize => window::latest().and_then(|id| window::minimize(id, true)),
        Message::Maximize => {
            state.is_maximized = !state.is_maximized;
            window::latest().and_then(|id| {
                window::is_maximized(id).then(move |is_maximized| window::maximize(id, !is_maximized))
            })
        }
        Message::SetMaximized(is_maximized) => {
            state.is_maximized = is_maximized;
            Task::none()
        }
        Message::Close => window::latest().and_then(window::close),
        Message::Drag => window::latest().and_then(window::drag),
    }
}
