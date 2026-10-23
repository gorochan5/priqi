#[derive(Debug, Clone)]
pub enum Message {
    Printers(crate::core::controls::printers::models::Message),
    Nothing
}