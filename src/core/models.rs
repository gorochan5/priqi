#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    Navbar(crate::core::navbar::models::Message),
}