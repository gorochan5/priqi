use iced::widget::image::Handle;

use crate::core::widgets::import_modal;

pub const RENAME_INPUT_ID: &str = "template_rename_input";

#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub zpl: String,
    pub width_mm: f32,
    pub height_mm: f32,
    pub dpi: u32,
    pub preview_handle: Option<Handle>,
}

impl Template {
    pub fn new(id: String, name: String, zpl: String) -> Self {
        let preview_handle = crate::core::renderer::render_zpl_to_handle(&zpl, 0).ok();
        Self {
            id,
            name,
            zpl,
            width_mm: 100.0,
            height_mm: 150.0,
            dpi: 203,
            preview_handle,
        }
    }

    pub fn refresh_preview(&mut self, resolved_zpl: Option<&str>) {
        let target = resolved_zpl.unwrap_or(&self.zpl);
        self.preview_handle = crate::core::renderer::render_zpl_to_handle(target, 0).ok();
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTemplate(String),
    DeleteTemplate(String),
    DuplicateTemplate,
    AddTemplate,
    StartRename(String),
    RenameInputChanged(String),
    SaveRename,
    CancelRename,
    ImportModal(import_modal::Message),
}

#[derive(Debug, Default)]
pub struct State {
    pub templates: Vec<Template>,
    pub selected_id: Option<String>,
    pub editing_id: Option<String>,
    pub editing_name: String,
    pub dirty_template_id: Option<String>,
    pub template_counter: usize,
    pub import_modal: import_modal::State,
}

impl State {
    pub fn take_dirty_id(&mut self) -> Option<String> {
        self.dirty_template_id.take()
    }
}
