use std::sync::Arc;

use crate::core::db::Db;

#[derive(Default)]
pub struct State {
    pub navbar: crate::core::navbar::models::State,
    pub controls: crate::core::controls::models::State,
    pub editor: crate::core::editor::models::State,
    pub templates: crate::core::templates::models::State,
    pub datasets: crate::core::datasets::models::State,
    pub db: Option<Arc<Db>>,
    pub db_error: Option<String>,
}

pub type LoadedDbData = (
    Arc<Db>,
    Vec<crate::core::templates::models::Template>,
    Vec<(String, String, crate::core::datasets::models::TableData)>,
);

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    ModifiersChanged(iced::keyboard::Modifiers),
    FileHovered(bool),
    FileDropped(std::path::PathBuf),
    CloseRequested(iced::window::Id),
    WindowResized(iced::window::Id),
    EscapePressed,
    PrintersLoaded(Vec<String>),
    DbOperationFinished(Result<(), String>),
    DismissDbError,
    Navbar(crate::core::navbar::models::Message),
    Controls(crate::core::controls::models::Message),
    Editor(crate::core::editor::models::Message),
    Templates(crate::core::templates::models::Message),
    Datasets(crate::core::datasets::models::Message),
    DbInitialized(Result<LoadedDbData, String>),
}
