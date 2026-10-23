#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    Navbar(crate::core::navbar::models::Message),
    Controls(crate::core::controls::models::Message),
}