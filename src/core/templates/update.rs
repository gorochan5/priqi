use iced::Task;

use crate::core::templates::models::{Message, SampleTemplateKind, State, Template};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectTemplate(id) => {
            state.selected_id = Some(id);
            Task::none()
        }
        Message::DeleteTemplate(id) => {
            state.templates.retain(|t| t.id != id);
            if state.selected_id.as_deref() == Some(&id) {
                state.selected_id = state.templates.first().map(|t| t.id.clone());
            }
            if state.editing_id.as_deref() == Some(&id) {
                state.editing_id = None;
                state.editing_name.clear();
            }
            Task::none()
        }
        Message::DuplicateTemplate => {
            if let Some((idx, source)) = state
                .selected_id
                .as_ref()
                .and_then(|id| state.templates.iter().enumerate().find(|(_, t)| &t.id == id))
            {
                let source = source.clone();
                state.template_counter += 1;
                let new_id = format!("{}_copia_{}", source.id, state.template_counter);
                let new_template = Template {
                    id: new_id.clone(),
                    name: format!("{} (Cópia)", source.name),
                    sample_kind: source.sample_kind,
                    zpl: source.zpl.clone(),
                };
                state.templates.insert(idx + 1, new_template);
                state.selected_id = Some(new_id);
            }
            Task::none()
        }
        Message::AddTemplate => {
            state.template_counter += 1;
            let new_id = format!("template_{}", state.template_counter);
            let new_name = format!("Nova Etiqueta {}", state.template_counter);
            let default_zpl = "^XA\n^FO50,50^A0N,30,30^FDNOVA ETIQUETA^FS\n^FO50,100^BY2^BCN,60,Y,N,N^FD12345678^FS\n^XZ".to_string();
            let new_template = Template {
                id: new_id.clone(),
                name: new_name,
                sample_kind: SampleTemplateKind::Custom,
                zpl: default_zpl,
            };
            state.templates.push(new_template);
            state.selected_id = Some(new_id);
            Task::none()
        }
        Message::StartRename(id) => {
            if let Some(tpl) = state.templates.iter().find(|t| t.id == id) {
                state.editing_id = Some(id);
                state.editing_name = tpl.name.clone();
            }
            Task::none()
        }
        Message::RenameInputChanged(val) => {
            state.editing_name = val;
            Task::none()
        }
        Message::SaveRename => {
            let trimmed = state.editing_name.trim();
            if !trimmed.is_empty()
                && let Some(tpl) = state
                    .editing_id
                    .as_ref()
                    .and_then(|id| state.templates.iter_mut().find(|t| &t.id == id))
            {
                tpl.name = trimmed.to_string();
            }
            state.editing_id = None;
            state.editing_name.clear();
            Task::none()
        }
        Message::CancelRename => {
            state.editing_id = None;
            state.editing_name.clear();
            Task::none()
        }
        Message::ImportModal(msg) => {
            crate::core::widgets::import_modal::update(&mut state.import_modal, msg)
                .map(Message::ImportModal)
        }
        Message::Nothing => Task::none(),
    }
}
