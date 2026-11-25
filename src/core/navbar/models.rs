#[derive(Debug, Clone, Default)]
pub struct State {
    pub is_maximized: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Minimize,
    Maximize,
    SetMaximized(bool),
    Close,
    Drag,
}
