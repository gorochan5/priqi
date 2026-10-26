#[derive(Default)]
pub struct State {
    pub controls: crate::core::controls::models::State,
    pub navbar: (),
    pub editor: crate::core::editor::models::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    Navbar(crate::core::navbar::models::Message),
    Controls(crate::core::controls::models::Message),
    Editor(crate::core::editor::models::Message),
}
