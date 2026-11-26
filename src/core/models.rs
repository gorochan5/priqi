use std::collections::HashMap;
use std::sync::Arc;

use iced::widget::image::Handle;

use crate::core::db::Db;
use crate::core::editor::models::PreviewCard;

#[derive(Default)]
pub struct State {
    pub navbar: crate::core::navbar::models::State,
    pub controls: crate::core::controls::models::State,
    pub editor: crate::core::editor::models::State,
    pub templates: crate::core::templates::models::State,
    pub datasets: crate::core::datasets::models::State,
    pub db: Option<Arc<Db>>,
    pub db_error: Option<String>,
    pub thumbnail_cache: HashMap<u64, Handle>,
    pub preview_cache: HashMap<u64, Vec<Handle>>,
    pub template_render_keys: HashMap<String, u64>,
    pub thumbnail_generation: u64,
    pub preview_generation: u64,
    pub dataset_load_generation: u64,
    pub preview_dirty: bool,
}

pub type LoadedDbData = (
    Arc<Db>,
    Vec<crate::core::templates::models::Template>,
    Vec<(String, String)>,
    Option<(String, Arc<crate::core::datasets::models::TableData>)>,
);

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(dead_code)]
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
    TemplatePreviewsRendered {
        generation: u64,
        results: Vec<(String, u64, Result<Handle, String>)>,
    },
    EditorPreviewRendered {
        generation: u64,
        cards: Vec<PreviewCard>,
        cache_entries: Vec<(u64, Vec<Handle>)>,
        error: Option<String>,
    },
    Navbar(crate::core::navbar::models::Message),
    Controls(crate::core::controls::models::Message),
    Editor(crate::core::editor::models::Message),
    Templates(crate::core::templates::models::Message),
    Datasets(crate::core::datasets::models::Message),
    DbInitialized(Result<LoadedDbData, String>),
    DatasetTableLoaded {
        generation: u64,
        id: String,
        result: Result<Option<Arc<crate::core::datasets::models::TableData>>, String>,
    },
}
