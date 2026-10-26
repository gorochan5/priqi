#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
}


#[derive(Debug, Clone, Default)]
pub struct State {
    pub nothing: String,
}