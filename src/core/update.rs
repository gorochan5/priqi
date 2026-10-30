use iced::Task;

use crate::core::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Navbar(msg) => {
            crate::core::navbar::update::update(&mut state.navbar, msg).map(Message::Navbar)
        }
        Message::Controls(msg) => {
            crate::core::controls::update::update(&mut state.controls, msg).map(Message::Controls)
        }
        Message::Editor(msg) => {
            let task =
                crate::core::editor::update::update(&mut state.editor, msg).map(Message::Editor);

            // Sincroniza as alterações de código feitas no editor com o template ativo
            if let Some(sel_id) = &state.templates.selected_id {
                let current_text = state.editor.content.text();
                if let Some(tpl) = state.templates.templates.iter_mut().find(|t| &t.id == sel_id) {
                    tpl.zpl = current_text;
                }
            }

            task
        }
        Message::Templates(msg) => {
            let prev_selected_id = state.templates.selected_id.clone();
            let task = crate::core::templates::update::update(&mut state.templates, msg)
                .map(Message::Templates);

            // Se o template selecionado mudou (ou novo adicionado/duplicado), atualiza o editor
            if state.templates.selected_id != prev_selected_id
                && let Some(tpl) = state
                    .templates
                    .selected_id
                    .as_ref()
                    .and_then(|id| state.templates.templates.iter().find(|t| &t.id == id))
            {
                state.editor.content = iced::widget::text_editor::Content::with_text(&tpl.zpl);
            }

            task
        }
        Message::Datasets(msg) => {
            crate::core::datasets::update::update(&mut state.datasets, msg).map(Message::Datasets)
        }
        Message::Nothing => Task::none(),
    }
}
