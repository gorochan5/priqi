use iced::Task;

use crate::core::templates::models::{Message, RENAME_INPUT_ID, State, Template};
use crate::core::widgets::import_modal;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectTemplate(id) => {
            clear_rename_state(state);
            state.selected_id = Some(id);
            Task::none()
        }
        Message::DeleteTemplate(id) => {
            state.templates.retain(|t| t.id != id);
            if state.selected_id.as_deref() == Some(&id) {
                state.selected_id = state.templates.first().map(|t| t.id.clone());
            }
            if state.editing_id.as_deref() == Some(&id) {
                clear_rename_state(state);
            }
            if state.dirty_template_id.as_deref() == Some(&id) {
                state.dirty_template_id = None;
            }
            Task::none()
        }
        Message::DuplicateTemplate => {
            clear_rename_state(state);
            if let Some((idx, source)) = state
                .selected_id
                .as_ref()
                .and_then(|id| state.templates.iter().enumerate().find(|(_, t)| &t.id == id))
            {
                let source = source.clone();
                let base_id = strip_copy_id_suffix(&source.id);
                let new_id = next_unique_template_id(state, &format!("{base_id}_copia"));
                let new_name = next_copy_name(&state.templates, &source.name);

                let mut new_template = Template::new(new_id.clone(), new_name, source.zpl.clone());
                new_template.width_mm = source.width_mm;
                new_template.height_mm = source.height_mm;
                new_template.dpi = source.dpi;

                state.templates.insert(idx + 1, new_template);
                state.selected_id = Some(new_id.clone());
                state.dirty_template_id = Some(new_id);
            }
            Task::none()
        }
        Message::AddTemplate => {
            clear_rename_state(state);
            let new_id = next_unique_template_id(state, "template");
            let new_name = format!("Nova Etiqueta {}", state.template_counter);
            let default_zpl = "^XA\n^FO50,50^A0N,30,30^FDNOVA ETIQUETA^FS\n^FO50,100^BY2^BCN,60,Y,N,N^FD12345678^FS\n^XZ".to_string();
            let new_template = Template::new(new_id.clone(), new_name, default_zpl);
            state.templates.push(new_template);
            state.selected_id = Some(new_id.clone());
            state.dirty_template_id = Some(new_id);
            Task::none()
        }
        Message::StartRename(id) => {
            if let Some(tpl) = state.templates.iter().find(|t| t.id == id) {
                state.editing_id = Some(id);
                state.editing_name = tpl.name.clone();
                return iced::widget::operation::focus(RENAME_INPUT_ID);
            }
            Task::none()
        }
        Message::RenameInputChanged(val) => {
            state.editing_name = val;
            Task::none()
        }
        Message::SaveRename => {
            let trimmed = state.editing_name.trim().to_string();
            if !trimmed.is_empty()
                && let Some(tpl) = state
                    .editing_id
                    .as_ref()
                    .and_then(|id| state.templates.iter_mut().find(|t| &t.id == id))
                && tpl.name != trimmed
            {
                tpl.name = trimmed;
                state.dirty_template_id = Some(tpl.id.clone());
            }
            clear_rename_state(state);
            Task::none()
        }
        Message::CancelRename => {
            clear_rename_state(state);
            Task::none()
        }
        Message::ImportModal(msg) => {
            state.import_modal.kind = Some(import_modal::models::ImportKind::Templates);
            if matches!(msg, import_modal::Message::Open) {
                clear_rename_state(state);
            }
            if matches!(msg, import_modal::Message::ImportPressed) {
                let zpl = state.import_modal.content.text();
                if !zpl.trim().is_empty() {
                    let new_id = next_unique_template_id(state, "template_import");
                    let new_name = state
                        .import_modal
                        .selected_file
                        .as_ref()
                        .and_then(|p| p.file_stem())
                        .and_then(|s| s.to_str())
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| {
                            format!("Etiqueta Importada {}", state.template_counter)
                        });
                    let new_template = Template::new(new_id.clone(), new_name, zpl);
                    state.templates.push(new_template);
                    state.selected_id = Some(new_id.clone());
                    state.dirty_template_id = Some(new_id);
                    state.import_modal.reset();
                } else {
                    state.import_modal.file_error = Some(
                        "Selecione um arquivo ou cole o código ZPL antes de importar".to_string(),
                    );
                }
                Task::none()
            } else {
                import_modal::update(&mut state.import_modal, msg).map(Message::ImportModal)
            }
        }
    }
}

fn clear_rename_state(state: &mut State) {
    state.editing_id = None;
    state.editing_name.clear();
}

fn next_unique_template_id(state: &mut State, prefix: &str) -> String {
    loop {
        state.template_counter += 1;
        let candidate = format!("{prefix}_{}", state.template_counter);
        if !state.templates.iter().any(|t| t.id == candidate) {
            return candidate;
        }
    }
}

fn strip_copy_id_suffix(id: &str) -> &str {
    if let Some((base, suffix)) = id.rsplit_once("_copia_")
        && !suffix.is_empty()
        && suffix.chars().all(|c| c.is_ascii_digit())
    {
        return base;
    }
    id.strip_suffix("_copia").unwrap_or(id)
}

fn strip_copy_name_suffix(name: &str) -> &str {
    let trimmed = name.trim_end();
    if let Some(base) = trimmed.strip_suffix(" (Cópia)") {
        return base;
    }
    if let Some((base, rest)) = trimmed.rsplit_once(" (Cópia ")
        && let Some(num) = rest.strip_suffix(')')
        && !num.is_empty()
        && num.chars().all(|c| c.is_ascii_digit())
    {
        return base;
    }
    trimmed
}

fn next_copy_name(templates: &[Template], source_name: &str) -> String {
    let base = strip_copy_name_suffix(source_name);
    let first = format!("{base} (Cópia)");
    if !templates.iter().any(|t| t.name == first) {
        return first;
    }
    let mut n = 2usize;
    loop {
        let candidate = format!("{base} (Cópia {n})");
        if !templates.iter().any(|t| t.name == candidate) {
            return candidate;
        }
        n += 1;
    }
}
