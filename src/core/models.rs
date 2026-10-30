pub struct State {
    pub navbar: (),
    pub controls: crate::core::controls::models::State,
    pub editor: crate::core::editor::models::State,
    pub templates: crate::core::templates::models::State,
    pub datasets: crate::core::datasets::models::State,
}

impl Default for State {
    fn default() -> Self {
        let templates = crate::core::templates::models::State::default();
        let mut editor = crate::core::editor::models::State::default();
        if let Some(tpl) = templates
            .selected_id
            .as_ref()
            .and_then(|id| templates.templates.iter().find(|t| &t.id == id))
        {
            editor.content = iced::widget::text_editor::Content::with_text(&tpl.zpl);
        }
        Self {
            navbar: (),
            controls: crate::core::controls::models::State::default(),
            editor,
            templates,
            datasets: crate::core::datasets::models::State::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    Navbar(crate::core::navbar::models::Message),
    Controls(crate::core::controls::models::Message),
    Editor(crate::core::editor::models::Message),
    Templates(crate::core::templates::models::Message),
    Datasets(crate::core::datasets::models::Message),
}
