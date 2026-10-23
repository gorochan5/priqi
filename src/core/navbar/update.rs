use iced::{Task, window};

use crate::core::navbar::models::Message;

pub fn update(_state: &mut (), message: Message) -> Task<Message>{
    match message {
        Message::Minimize => {
            window::latest().and_then(|id| window::minimize(id, true))
        }
        Message::Maximize => {
            window::latest().and_then(|id| {
                window::is_maximized(id).then(move |is_maximized| window::maximize(id, !is_maximized))
            })
        }
        Message::Close => {
            window::latest().and_then(window::close)
        }    
        
        Message::Drag => {
            window::latest().and_then(window::drag)
        }
    }
}
