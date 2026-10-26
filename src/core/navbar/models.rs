#[derive(Debug, Clone)]
pub enum Message {
    Minimize,
    Maximize,
    Close,
    Drag,
}
