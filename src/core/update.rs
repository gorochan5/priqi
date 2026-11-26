use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use iced::Task;
use iced::widget::image::Handle;

use crate::core::controls::models::PrinterOption;
use crate::core::editor::models::{PreviewCard, Tab};
use crate::core::models::{Message, State};
use crate::core::printer::PrintDestination;
use crate::core::renderer::LabelRenderOptions;
use crate::core::widgets::import_modal;

const MAX_CACHE_ENTRIES: usize = 64;
const MAX_PREVIEW_ROWS: usize = 12;
const THUMBNAIL_MAX_DIM: u32 = 360;

enum TemplatePreviewScope {
    All,
    DynamicOnly,
    Single(String),
}

struct PreviewRowSpec {
    base_title: String,
    subtitle: String,
    resolved_zpl: String,
    cache_key: u64,
    cached_handles: Option<Vec<Handle>>,
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ModifiersChanged(modifiers) => {
            state.datasets.shift_pressed = modifiers.shift();
            Task::none()
        }
        Message::WindowResized(id) => iced::window::is_maximized(id)
            .map(|maximized| Message::Navbar(crate::core::navbar::models::Message::SetMaximized(maximized))),
        Message::CloseRequested(id) => {
            let flush_task = flush_pending_edits(state, false);
            flush_task.chain(iced::window::close(id))
        }
        Message::EscapePressed => {
            if state.templates.import_modal.is_open {
                update(
                    state,
                    Message::Templates(crate::core::templates::models::Message::ImportModal(
                        import_modal::Message::Close,
                    )),
                )
            } else if state.datasets.import_modal.is_open {
                update(
                    state,
                    Message::Datasets(crate::core::datasets::models::Message::ImportModal(
                        import_modal::Message::Close,
                    )),
                )
            } else if state.datasets.editing_cell.is_some() {
                update(
                    state,
                    Message::Datasets(crate::core::datasets::models::Message::CancelEditing),
                )
            } else if state.datasets.renaming_dataset_id.is_some() {
                update(
                    state,
                    Message::Datasets(crate::core::datasets::models::Message::CancelDatasetRename),
                )
            } else if state.datasets.is_dropdown_open {
                update(
                    state,
                    Message::Datasets(crate::core::datasets::models::Message::CloseDatasetDropdown),
                )
            } else if state.templates.editing_id.is_some() {
                update(
                    state,
                    Message::Templates(crate::core::templates::models::Message::CancelRename),
                )
            } else {
                Task::none()
            }
        }
        Message::PrintersLoaded(printers) => {
            let mut options: Vec<PrinterOption> = printers
                .into_iter()
                .map(PrinterOption::OsPrinter)
                .collect();
            options.push(PrinterOption::Pdf);
            state.controls.available_printers = options;

            let keep_current = state
                .controls
                .printer
                .as_ref()
                .is_some_and(|p| state.controls.available_printers.contains(p));
            if !keep_current {
                state.controls.printer = state.controls.available_printers.first().cloned();
            }
            Task::none()
        }
        Message::DbOperationFinished(res) => {
            if let Err(err) = res {
                tracing::error!("{err}");
                state.db_error = Some(err);
            }
            Task::none()
        }
        Message::DismissDbError => {
            state.db_error = None;
            Task::none()
        }
        Message::TemplatePreviewsRendered {
            generation,
            results,
        } => {
            if generation != state.thumbnail_generation {
                return Task::none();
            }
            for (template_id, cache_key, result) in results {
                match result {
                    Ok(handle) => {
                        insert_bounded(&mut state.thumbnail_cache, cache_key, handle.clone());
                        if !state.template_render_keys.contains_key(&template_id)
                            && let Some(tpl) = state
                                .templates
                                .templates
                                .iter_mut()
                                .find(|t| t.id == template_id)
                        {
                            tpl.preview_handle = Some(handle);
                            state.template_render_keys.insert(template_id, cache_key);
                        }
                    }
                    Err(_) => {
                        if !state.template_render_keys.contains_key(&template_id)
                            && let Some(tpl) = state
                                .templates
                                .templates
                                .iter_mut()
                                .find(|t| t.id == template_id)
                        {
                            tpl.preview_handle = None;
                            state.template_render_keys.insert(template_id, cache_key);
                        }
                    }
                }
            }
            Task::none()
        }
        Message::EditorPreviewRendered {
            generation,
            cards,
            cache_entries,
            error,
        } => {
            if generation != state.preview_generation {
                return Task::none();
            }
            for (key, handles) in cache_entries {
                insert_bounded(&mut state.preview_cache, key, handles);
            }
            apply_preview_cards(state, cards, error);
            state.preview_dirty = false;
            Task::none()
        }
        Message::FileHovered(hovered) => {
            if state.templates.import_modal.is_open {
                import_modal::update(
                    &mut state.templates.import_modal,
                    import_modal::Message::FileHovered(hovered),
                )
                .map(crate::core::templates::models::Message::ImportModal)
                .map(Message::Templates)
            } else if state.datasets.import_modal.is_open {
                import_modal::update(
                    &mut state.datasets.import_modal,
                    import_modal::Message::FileHovered(hovered),
                )
                .map(crate::core::datasets::models::Message::ImportModal)
                .map(Message::Datasets)
            } else {
                Task::none()
            }
        }
        Message::FileDropped(path) => {
            if state.templates.import_modal.is_open {
                import_modal::update(
                    &mut state.templates.import_modal,
                    import_modal::Message::FileDropped(path),
                )
                .map(crate::core::templates::models::Message::ImportModal)
                .map(Message::Templates)
            } else if state.datasets.import_modal.is_open {
                import_modal::update(
                    &mut state.datasets.import_modal,
                    import_modal::Message::FileDropped(path),
                )
                .map(crate::core::datasets::models::Message::ImportModal)
                .map(Message::Datasets)
            } else {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_ascii_lowercase());
                match ext.as_deref() {
                    Some("zpl" | "prn" | "txt") => {
                        let open_task = update(
                            state,
                            Message::Templates(
                                crate::core::templates::models::Message::ImportModal(
                                    import_modal::Message::Open,
                                ),
                            ),
                        );
                        let drop_task = update(
                            state,
                            Message::Templates(
                                crate::core::templates::models::Message::ImportModal(
                                    import_modal::Message::FileDropped(path),
                                ),
                            ),
                        );
                        Task::batch([open_task, drop_task])
                    }
                    Some("csv") => {
                        let open_task = update(
                            state,
                            Message::Datasets(
                                crate::core::datasets::models::Message::ImportModal(
                                    import_modal::Message::Open,
                                ),
                            ),
                        );
                        let drop_task = update(
                            state,
                            Message::Datasets(
                                crate::core::datasets::models::Message::ImportModal(
                                    import_modal::Message::FileDropped(path),
                                ),
                            ),
                        );
                        Task::batch([open_task, drop_task])
                    }
                    _ => Task::none(),
                }
            }
        }
        Message::DbInitialized(res) => match res {
            Ok((db, templates, dataset_meta, initial_table)) => {
                state.db = Some(db);
                state.db_error = None;
                state.template_render_keys.clear();

                state.templates.templates = templates;
                state.templates.template_counter =
                    max_id_suffix(state.templates.templates.iter().map(|t| t.id.as_str()));
                state.templates.selected_id =
                    state.templates.templates.first().map(|t| t.id.clone());

                state.datasets.options.clear();
                state.datasets.tables.clear();
                for (id, name) in dataset_meta {
                    state.datasets.options.push(
                        crate::core::datasets::models::DatasetOption { id, name },
                    );
                }
                if let Some((id, table)) = initial_table {
                    state.datasets.tables.insert(id, table);
                }
                state.datasets.dataset_counter =
                    max_id_suffix(state.datasets.options.iter().map(|d| d.id.as_str()));
                state.datasets.dataset = state.datasets.options.first().cloned();
                state.datasets.is_loading_dataset = false;
                state.datasets.selected_rows.clear();
                state.datasets.last_selected_row = None;
                if let Some(t) = state.datasets.current_table()
                    && !t.rows.is_empty()
                {
                    state.datasets.selected_rows.insert(0);
                    state.datasets.last_selected_row = Some(0);
                }
                state.datasets.rebuild_filtered_indices();

                sync_editor_from_selected_template(state);
                sync_can_print(state);

                let thumb_task = schedule_template_previews(state, TemplatePreviewScope::All);
                let preview_task = schedule_editor_preview(state);
                let printers_task = Task::perform(
                    async {
                        tokio::task::spawn_blocking(
                            crate::core::printer::enumerate_windows_printers,
                        )
                        .await
                        .unwrap_or_default()
                    },
                    Message::PrintersLoaded,
                );

                Task::batch([thumb_task, preview_task, printers_task])
            }
            Err(err) => {
                tracing::error!("Falha ao inicializar banco: {err}");
                state.db_error = Some(format!("Erro ao abrir banco de dados: {err}"));
                Task::none()
            }
        },
        Message::Navbar(msg) => {
            if matches!(msg, crate::core::navbar::models::Message::Close) {
                let flush_task = flush_pending_edits(state, false);
                return flush_task.chain(iced::window::latest().and_then(iced::window::close));
            }
            let blur_task = commit_editor_if_focused(state);
            let nav_task =
                crate::core::navbar::update::update(&mut state.navbar, msg).map(Message::Navbar);
            Task::batch([blur_task, nav_task])
        }
        Message::Controls(msg) => {
            if matches!(msg, crate::core::controls::models::Message::PrintPressed) {
                let flush_task = flush_pending_edits(state, true);

                let has_rows_but_none_selected = state
                    .datasets
                    .current_table()
                    .is_some_and(|t| !t.rows.is_empty())
                    && state.datasets.selected_rows.is_empty();

                if has_rows_but_none_selected {
                    let err_task = crate::core::controls::update::update(
                        &mut state.controls,
                        crate::core::controls::models::Message::PrintResult(Err(
                            "Selecione ao menos uma linha para imprimir.".to_string(),
                        )),
                    )
                    .map(Message::Controls);
                    let dismiss_task = spawn_print_status_auto_dismiss(state.controls.print_status_seq);
                    return Task::batch([flush_task, err_task, dismiss_task]);
                }

                let ctrl_task = crate::core::controls::update::update(
                    &mut state.controls,
                    crate::core::controls::models::Message::PrintPressed,
                )
                .map(Message::Controls);

                let template_zpl = state.editor.content.text();
                let (headers, rows) = if let Some(table) = state.datasets.current_table() {
                    let mut selected = Vec::with_capacity(state.datasets.selected_rows.len());
                    for &idx in &state.datasets.selected_rows {
                        if let Some(r) = table.rows.get(idx) {
                            selected.push(r.clone());
                        }
                    }
                    (table.headers.clone(), selected)
                } else {
                    (Vec::new(), Vec::new())
                };

                let copies = state.controls.copies;
                let destination = match state
                    .controls
                    .printer
                    .clone()
                    .unwrap_or(PrinterOption::Pdf)
                {
                    PrinterOption::OsPrinter(name) => PrintDestination::OsPrinter(name),
                    PrinterOption::Pdf => PrintDestination::Pdf,
                };
                let render_opts = current_render_options(state);
                let rotation = state.editor.rotation;

                let print_task = Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::printer::execute_print_job(
                                &template_zpl,
                                &headers,
                                &rows,
                                copies,
                                destination,
                                render_opts,
                                rotation,
                            )
                        })
                        .await
                        .unwrap_or_else(|e| Err(format!("Erro interno de impressão: {e}")))
                    },
                    |res| {
                        Message::Controls(crate::core::controls::models::Message::PrintResult(res))
                    },
                );

                Task::batch([flush_task, ctrl_task, print_task])
            } else {
                let is_print_result = matches!(
                    msg,
                    crate::core::controls::models::Message::PrintResult(_)
                );
                let blur_task = commit_editor_if_focused(state);
                let ctrl_task = crate::core::controls::update::update(&mut state.controls, msg)
                    .map(Message::Controls);
                if is_print_result {
                    let dismiss_task =
                        spawn_print_status_auto_dismiss(state.controls.print_status_seq);
                    Task::batch([blur_task, ctrl_task, dismiss_task])
                } else {
                    Task::batch([blur_task, ctrl_task])
                }
            }
        }
        Message::Editor(msg) => {
            let prev_tab = state.editor.active_tab;
            let is_unfocus_or_tab = matches!(
                msg,
                crate::core::editor::models::Message::Unfocus
                    | crate::core::editor::models::Message::SelectTab(_)
            );
            let is_content_edit = matches!(
                &msg,
                crate::core::editor::models::Message::Edit(action) if action.is_edit()
            );
            let is_label_config = matches!(
                msg,
                crate::core::editor::models::Message::LabelWidthChanged(_)
                    | crate::core::editor::models::Message::LabelHeightChanged(_)
                    | crate::core::editor::models::Message::LabelDpiSelected(_)
            );

            let task =
                crate::core::editor::update::update(&mut state.editor, msg).map(Message::Editor);

            let mut extra_tasks = Vec::new();
            let switched_to_preview =
                prev_tab != state.editor.active_tab && state.editor.active_tab == Tab::Preview;

            if is_content_edit {
                sync_can_print(state);
                state.preview_dirty = true;
            }

            if is_unfocus_or_tab {
                if state.editor.is_dirty {
                    state.editor.is_dirty = false;
                    extra_tasks.push(sync_active_template_and_previews(state));
                } else if switched_to_preview
                    && (state.preview_dirty || state.editor.preview_cards.is_empty())
                {
                    extra_tasks.push(schedule_editor_preview(state));
                }
            } else if is_label_config
                && let Some(sel_id) = state.templates.selected_id.clone()
                && let Some(tpl) = state
                    .templates
                    .templates
                    .iter_mut()
                    .find(|t| t.id == sel_id)
            {
                let changed = (tpl.width_mm - state.editor.label_width_mm).abs() > f32::EPSILON
                    || (tpl.height_mm - state.editor.label_height_mm).abs() > f32::EPSILON
                    || tpl.dpi != state.editor.label_dpi;
                if changed {
                    tpl.width_mm = state.editor.label_width_mm;
                    tpl.height_mm = state.editor.label_height_mm;
                    tpl.dpi = state.editor.label_dpi;
                    if let Some(db) = &state.db {
                        extra_tasks.push(spawn_save_template(db.clone(), tpl.clone()));
                    }
                    extra_tasks.push(schedule_template_previews(
                        state,
                        TemplatePreviewScope::Single(sel_id),
                    ));
                    extra_tasks.push(schedule_editor_preview(state));
                }
            }

            if extra_tasks.is_empty() {
                task
            } else {
                extra_tasks.insert(0, task);
                Task::batch(extra_tasks)
            }
        }
        Message::Templates(msg) => {
            let blur_task = commit_editor_if_focused(state);
            let prev_selected_id = state.templates.selected_id.clone();
            let is_rename = matches!(msg, crate::core::templates::models::Message::SaveRename);
            let delete_id = match &msg {
                crate::core::templates::models::Message::DeleteTemplate(id) => Some(id.clone()),
                _ => None,
            };

            let task = crate::core::templates::update::update(&mut state.templates, msg)
                .map(Message::Templates);

            let dirty_id = state.templates.take_dirty_id();
            let mut db_task = Task::none();
            let mut thumb_task = Task::none();
            let mut preview_task = Task::none();

            if let Some(del_id) = &delete_id {
                state.template_render_keys.remove(del_id);
                if let Some(db) = &state.db {
                    db_task = spawn_delete_template(db.clone(), del_id.clone());
                }
            } else if let Some(d_id) = &dirty_id {
                if let Some(tpl) = state.templates.templates.iter().find(|t| &t.id == d_id)
                    && let Some(db) = &state.db
                {
                    db_task = spawn_save_template(db.clone(), tpl.clone());
                }
                if !is_rename {
                    thumb_task = schedule_template_previews(
                        state,
                        TemplatePreviewScope::Single(d_id.clone()),
                    );
                }
            }

            let selection_changed = state.templates.selected_id != prev_selected_id;
            if selection_changed {
                sync_editor_from_selected_template(state);
                sync_can_print(state);
                preview_task = schedule_editor_preview(state);
            }

            Task::batch([blur_task, task, db_task, thumb_task, preview_task])
        }
        Message::Datasets(msg) => {
            if matches!(
                msg,
                crate::core::datasets::models::Message::TableScrolled { .. }
            ) {
                return crate::core::datasets::update::update(&mut state.datasets, msg)
                    .map(Message::Datasets);
            }

            let blur_task = commit_editor_if_focused(state);
            let prev_selection_version = state.datasets.selection_version;

            let task = crate::core::datasets::update::update(&mut state.datasets, msg)
                .map(Message::Datasets);

            let dirty_ids = state.datasets.take_dirty_ids();
            let deleted_ids = state.datasets.take_deleted_ids();
            let renamed_datasets = state.datasets.take_renamed_datasets();

            let mut all_tasks = vec![blur_task, task];
            if let Some(db) = &state.db {
                for id in &dirty_ids {
                    if let Some(opt) = state.datasets.options.iter().find(|o| &o.id == id)
                        && let Some(table) = state.datasets.tables.get(id)
                    {
                        all_tasks.push(spawn_save_dataset(
                            db.clone(),
                            id.clone(),
                            opt.name.clone(),
                            std::sync::Arc::clone(table),
                        ));
                    }
                }
                for (id, name) in renamed_datasets {
                    if !dirty_ids.contains(&id) {
                        all_tasks.push(spawn_rename_dataset(db.clone(), id, name));
                    }
                }
                for id in &deleted_ids {
                    all_tasks.push(spawn_delete_dataset(db.clone(), id.clone()));
                }
            }

            let active_id = state.datasets.dataset.as_ref().map(|d| d.id.clone());
            if let Some(active_id) = active_id {
                if state.datasets.tables.contains_key(&active_id) {
                    state.datasets.is_loading_dataset = false;
                    state.datasets.evict_inactive_tables();
                } else if let Some(db) = &state.db {
                    state.datasets.is_loading_dataset = true;
                    state.dataset_load_generation = state.dataset_load_generation.wrapping_add(1);
                    all_tasks.push(spawn_load_dataset_table(
                        db.clone(),
                        state.dataset_load_generation,
                        active_id,
                    ));
                }
            } else {
                state.datasets.is_loading_dataset = false;
                state.datasets.evict_inactive_tables();
            }

            if state.datasets.selection_version != prev_selection_version {
                all_tasks.push(schedule_template_previews(
                    state,
                    TemplatePreviewScope::DynamicOnly,
                ));
                all_tasks.push(schedule_editor_preview(state));
            }

            Task::batch(all_tasks)
        }
        Message::DatasetTableLoaded {
            generation,
            id,
            result,
        } => {
            if generation != state.dataset_load_generation
                || state.datasets.dataset.as_ref().map(|d| d.id.as_str()) != Some(id.as_str())
            {
                return Task::none();
            }

            state.datasets.is_loading_dataset = false;
            match result {
                Ok(Some(table)) => {
                    state.datasets.tables.insert(id, table);
                    state.datasets.evict_inactive_tables();
                    state.datasets.selected_rows.clear();
                    state.datasets.last_selected_row = None;
                    if state
                        .datasets
                        .current_table()
                        .is_some_and(|t| !t.rows.is_empty())
                    {
                        state.datasets.selected_rows.insert(0);
                        state.datasets.last_selected_row = Some(0);
                    }
                    state.datasets.rebuild_filtered_indices();
                    state.datasets.bump_selection_version();
                    Task::batch([
                        schedule_template_previews(state, TemplatePreviewScope::DynamicOnly),
                        schedule_editor_preview(state),
                    ])
                }
                Ok(None) => Task::none(),
                Err(err) => {
                    tracing::error!("{err}");
                    state.db_error = Some(err);
                    Task::none()
                }
            }
        }
        Message::Nothing => Task::none(),
    }
}

fn max_id_suffix<'a>(ids: impl Iterator<Item = &'a str>) -> usize {
    ids.filter_map(|id| id.rsplit_once('_').and_then(|(_, s)| s.parse::<usize>().ok()))
        .max()
        .unwrap_or(0)
}

fn format_mm(val: f32) -> String {
    if (val - val.round()).abs() < 0.01 {
        format!("{}", val.round() as i32)
    } else {
        format!("{val:.1}")
    }
}

fn compute_render_cache_key(resolved_zpl: &str, opts: LabelRenderOptions) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    resolved_zpl.hash(&mut hasher);
    opts.width_mm.to_bits().hash(&mut hasher);
    opts.height_mm.to_bits().hash(&mut hasher);
    opts.dpi.hash(&mut hasher);
    hasher.finish()
}

fn insert_bounded<V>(cache: &mut HashMap<u64, V>, key: u64, value: V) {
    if cache.len() >= MAX_CACHE_ENTRIES && !cache.contains_key(&key) {
        cache.clear();
    }
    cache.insert(key, value);
}

fn editor_has_non_whitespace(content: &iced::widget::text_editor::Content) -> bool {
    !content.is_empty() && content.lines().any(|line| !line.text.trim().is_empty())
}

fn sync_editor_from_selected_template(state: &mut State) {
    if let Some(tpl) = state
        .templates
        .selected_id
        .as_ref()
        .and_then(|id| state.templates.templates.iter().find(|t| &t.id == id))
    {
        state.editor.content = iced::widget::text_editor::Content::with_text(&tpl.zpl);
        state.editor.has_template_selected = true;
        state.editor.label_width_mm = tpl.width_mm;
        state.editor.label_height_mm = tpl.height_mm;
        state.editor.label_dpi = tpl.dpi;
        state.editor.width_input = format_mm(tpl.width_mm);
        state.editor.height_input = format_mm(tpl.height_mm);
    } else {
        state.editor.content = iced::widget::text_editor::Content::new();
        state.editor.has_template_selected = false;
    }
    state.editor.is_dirty = false;
    state.editor.sync_line_numbers();
}

fn sync_can_print(state: &mut State) {
    state.controls.can_print = editor_has_non_whitespace(&state.editor.content);
}

fn current_render_options(state: &State) -> LabelRenderOptions {
    LabelRenderOptions {
        width_mm: state.editor.label_width_mm,
        height_mm: state.editor.label_height_mm,
        dpi: state.editor.label_dpi,
    }
}

fn flush_pending_edits(state: &mut State, schedule_renders: bool) -> Task<Message> {
    let mut tasks = Vec::new();

    if state.editor.is_dirty || state.editor.is_focused {
        state.editor.is_focused = false;
        let was_dirty = state.editor.is_dirty;
        state.editor.is_dirty = false;
        if was_dirty {
            if schedule_renders {
                tasks.push(sync_active_template_and_previews(state));
            } else if let Some(sel_id) = &state.templates.selected_id {
                let current_text = state.editor.content.text();
                if let Some(tpl) = state
                    .templates
                    .templates
                    .iter_mut()
                    .find(|t| &t.id == sel_id)
                {
                    tpl.zpl = current_text;
                    if let Some(db) = &state.db {
                        tasks.push(spawn_save_template(db.clone(), tpl.clone()));
                    }
                }
            }
        }
    }

    if state.datasets.editing_cell.is_some() {
        let _ = crate::core::datasets::update::update(
            &mut state.datasets,
            crate::core::datasets::models::Message::SaveCell,
        );
    }

    if state.datasets.renaming_dataset_id.is_some() {
        let _ = crate::core::datasets::update::update(
            &mut state.datasets,
            crate::core::datasets::models::Message::SaveDatasetRename,
        );
    }

    if state.templates.editing_id.is_some() {
        let _ = crate::core::templates::update::update(
            &mut state.templates,
            crate::core::templates::models::Message::SaveRename,
        );
    }

    if let Some(db) = &state.db {
        if let Some(tpl_id) = state.templates.take_dirty_id()
            && let Some(tpl) = state.templates.templates.iter().find(|t| t.id == tpl_id)
        {
            tasks.push(spawn_save_template(db.clone(), tpl.clone()));
        }

        let dirty_ids = state.datasets.take_dirty_ids();
        for id in &dirty_ids {
            if let Some(opt) = state.datasets.options.iter().find(|o| &o.id == id)
                && let Some(table) = state.datasets.tables.get(id)
            {
                tasks.push(spawn_save_dataset(
                    db.clone(),
                    id.clone(),
                    opt.name.clone(),
                    std::sync::Arc::clone(table),
                ));
            }
        }

        for (id, name) in state.datasets.take_renamed_datasets() {
            if !dirty_ids.contains(&id) {
                tasks.push(spawn_rename_dataset(db.clone(), id, name));
            }
        }

        for id in state.datasets.take_deleted_ids() {
            tasks.push(spawn_delete_dataset(db.clone(), id));
        }
    }

    Task::batch(tasks)
}

fn commit_editor_if_focused(state: &mut State) -> Task<Message> {
    if state.editor.is_focused {
        state.editor.is_focused = false;
        if state.editor.is_dirty {
            state.editor.is_dirty = false;
            return sync_active_template_and_previews(state);
        }
    }
    Task::none()
}

fn sync_active_template_and_previews(state: &mut State) -> Task<Message> {
    let mut save_task = Task::none();
    let mut thumb_task = Task::none();

    if let Some(sel_id) = state.templates.selected_id.clone() {
        let current_text = state.editor.content.text();
        if let Some(tpl) = state
            .templates
            .templates
            .iter_mut()
            .find(|t| t.id == sel_id)
        {
            tpl.zpl = current_text;
            if let Some(db) = &state.db {
                save_task = spawn_save_template(db.clone(), tpl.clone());
            }
        }
        thumb_task = schedule_template_previews(state, TemplatePreviewScope::Single(sel_id));
    }

    let preview_task = schedule_editor_preview(state);
    Task::batch([save_task, thumb_task, preview_task])
}

fn schedule_template_previews(state: &mut State, scope: TemplatePreviewScope) -> Task<Message> {
    let first_row: Option<(&[String], &[String])> =
        state.datasets.current_table().and_then(|table| {
            let row_idx = state.datasets.first_selected_row()?;
            table
                .rows
                .get(row_idx)
                .map(|r| (table.headers.as_slice(), r.as_slice()))
        });

    let mut jobs: Vec<(String, u64, String, LabelRenderOptions)> = Vec::new();

    for tpl in &mut state.templates.templates {
        let is_unrendered = !state.template_render_keys.contains_key(&tpl.id);
        let in_scope = match &scope {
            TemplatePreviewScope::All => true,
            TemplatePreviewScope::DynamicOnly => {
                tpl.zpl.contains('[') || tpl.preview_handle.is_none() || is_unrendered
            }
            TemplatePreviewScope::Single(id) => &tpl.id == id || is_unrendered,
        };
        if !in_scope {
            continue;
        }

        let resolved: Cow<'_, str> = if let Some((headers, row)) = first_row
            && tpl.zpl.contains('[')
        {
            Cow::Owned(crate::core::renderer::substitute_template(
                &tpl.zpl, headers, row,
            ))
        } else {
            Cow::Borrowed(tpl.zpl.as_str())
        };

        let opts = LabelRenderOptions {
            width_mm: tpl.width_mm,
            height_mm: tpl.height_mm,
            dpi: tpl.dpi,
        };
        let cache_key = compute_render_cache_key(&resolved, opts);

        if state.template_render_keys.get(&tpl.id) == Some(&cache_key) {
            continue;
        }

        if resolved.trim().is_empty() {
            tpl.preview_handle = None;
            state.template_render_keys.insert(tpl.id.clone(), cache_key);
            continue;
        }

        if let Some(cached) = state.thumbnail_cache.get(&cache_key) {
            tpl.preview_handle = Some(cached.clone());
            state.template_render_keys.insert(tpl.id.clone(), cache_key);
            continue;
        }

        state.template_render_keys.remove(&tpl.id);
        jobs.push((tpl.id.clone(), cache_key, resolved.into_owned(), opts));
    }

    if jobs.is_empty() {
        return Task::none();
    }

    state.thumbnail_generation = state.thumbnail_generation.wrapping_add(1);
    let generation = state.thumbnail_generation;

    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                let mut results: Vec<(String, u64, Result<Handle, String>)> =
                    Vec::with_capacity(jobs.len());
                let mut batch_cache: HashMap<u64, Handle> = HashMap::new();

                for (tpl_id, cache_key, zpl, opts) in jobs {
                    if let Some(existing) = batch_cache.get(&cache_key) {
                        results.push((tpl_id, cache_key, Ok(existing.clone())));
                        continue;
                    }

                    let rendered = crate::core::renderer::render_zpl_to_thumbnail_handle(
                        &zpl,
                        opts,
                        THUMBNAIL_MAX_DIM,
                    );
                    if let Ok(ref handle) = rendered {
                        batch_cache.insert(cache_key, handle.clone());
                    }
                    results.push((tpl_id, cache_key, rendered));
                }

                results
            })
            .await
            .unwrap_or_default()
        },
        move |results| Message::TemplatePreviewsRendered {
            generation,
            results,
        },
    )
}

fn push_preview_cards(
    cards: &mut Vec<PreviewCard>,
    base_title: &str,
    subtitle: &str,
    handles: Vec<Handle>,
) {
    let total = handles.len();
    for (idx, handle) in handles.into_iter().enumerate() {
        let title = if total > 1 {
            format!("{base_title} ({}/{total})", idx + 1)
        } else {
            base_title.to_string()
        };
        cards.push(PreviewCard {
            title,
            subtitle: subtitle.to_string(),
            image: Some(handle),
            error: None,
        });
    }
}

fn apply_preview_cards(state: &mut State, cards: Vec<PreviewCard>, error: Option<String>) {
    if cards.len() <= 1 {
        state.editor.maximized_card = None;
    } else if let Some(max_idx) = state.editor.maximized_card
        && max_idx >= cards.len()
    {
        state.editor.maximized_card = Some(cards.len() - 1);
    }

    state.editor.preview_cards = cards;
    state.editor.preview_error = error;
}

fn schedule_editor_preview(state: &mut State) -> Task<Message> {
    if state.editor.active_tab != Tab::Preview {
        state.preview_dirty = true;
        state.preview_generation = state.preview_generation.wrapping_add(1);
        return Task::none();
    }

    if !editor_has_non_whitespace(&state.editor.content) {
        state.preview_generation = state.preview_generation.wrapping_add(1);
        state.editor.preview_cards.clear();
        state.editor.preview_error = None;
        state.editor.maximized_card = None;
        state.preview_dirty = false;
        return Task::none();
    }

    let raw_zpl = state.editor.content.text();
    let render_opts = current_render_options(state);
    let mut specs: Vec<PreviewRowSpec> = Vec::new();

    if let Some(table) = state.datasets.current_table()
        && !state.datasets.selected_rows.is_empty()
    {
        for &row_idx in state.datasets.selected_rows.iter().take(MAX_PREVIEW_ROWS) {
            if let Some(row) = table.rows.get(row_idx) {
                let resolved = if raw_zpl.contains('[') {
                    crate::core::renderer::substitute_template(&raw_zpl, &table.headers, row)
                } else {
                    raw_zpl.clone()
                };
                let first_val = row
                    .first()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty());

                let base_title = match first_val {
                    Some(v) => format!("Linha {} — {v}", row_idx + 1),
                    None => format!("Linha {}", row_idx + 1),
                };

                let subtitle = table
                    .headers
                    .iter()
                    .zip(row.iter())
                    .take(3)
                    .map(|(h, v)| format!("{h}: {v}"))
                    .collect::<Vec<_>>()
                    .join("  •  ");

                let cache_key = compute_render_cache_key(&resolved, render_opts);
                let cached_handles = state.preview_cache.get(&cache_key).cloned();

                specs.push(PreviewRowSpec {
                    base_title,
                    subtitle,
                    resolved_zpl: resolved,
                    cache_key,
                    cached_handles,
                });
            }
        }
    }

    if specs.is_empty() {
        let cache_key = compute_render_cache_key(&raw_zpl, render_opts);
        let cached_handles = state.preview_cache.get(&cache_key).cloned();
        specs.push(PreviewRowSpec {
            base_title: "Visualização do Modelo".to_string(),
            subtitle: String::new(),
            resolved_zpl: raw_zpl,
            cache_key,
            cached_handles,
        });
    }

    if specs.iter().all(|s| s.cached_handles.is_some()) {
        state.preview_generation = state.preview_generation.wrapping_add(1);
        let mut cards = Vec::with_capacity(specs.len());
        for spec in specs {
            if let Some(handles) = spec.cached_handles {
                push_preview_cards(&mut cards, &spec.base_title, &spec.subtitle, handles);
            }
        }
        apply_preview_cards(state, cards, None);
        state.preview_dirty = false;
        return Task::none();
    }

    state.preview_generation = state.preview_generation.wrapping_add(1);
    let generation = state.preview_generation;

    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                let mut cards = Vec::with_capacity(specs.len());
                let mut cache_entries: Vec<(u64, Vec<Handle>)> = Vec::new();
                let mut first_error: Option<String> = None;

                for spec in specs {
                    if let Some(handles) = spec.cached_handles {
                        push_preview_cards(&mut cards, &spec.base_title, &spec.subtitle, handles);
                        continue;
                    }

                    if let Some((_, existing)) =
                        cache_entries.iter().find(|(k, _)| *k == spec.cache_key)
                    {
                        push_preview_cards(
                            &mut cards,
                            &spec.base_title,
                            &spec.subtitle,
                            existing.clone(),
                        );
                        continue;
                    }

                    match crate::core::renderer::render_zpl_all_to_preview_handles(
                        &spec.resolved_zpl,
                        render_opts,
                    ) {
                        Ok(handles) => {
                            cache_entries.push((spec.cache_key, handles.clone()));
                            push_preview_cards(
                                &mut cards,
                                &spec.base_title,
                                &spec.subtitle,
                                handles,
                            );
                        }
                        Err(err) => {
                            if first_error.is_none() {
                                first_error = Some(err.clone());
                            }
                            cards.push(PreviewCard {
                                title: spec.base_title,
                                subtitle: spec.subtitle,
                                image: None,
                                error: Some(err),
                            });
                        }
                    }
                }

                (cards, cache_entries, first_error)
            })
            .await
            .unwrap_or_else(|e| {
                let msg = format!("Erro interno de renderização: {e}");
                (
                    vec![PreviewCard {
                        title: "Visualização do Modelo".to_string(),
                        subtitle: String::new(),
                        image: None,
                        error: Some(msg.clone()),
                    }],
                    Vec::new(),
                    Some(msg),
                )
            })
        },
        move |(cards, cache_entries, error)| Message::EditorPreviewRendered {
            generation,
            cards,
            cache_entries,
            error,
        },
    )
}

fn spawn_save_template(
    db: std::sync::Arc<crate::core::db::Db>,
    template: crate::core::templates::models::Template,
) -> Task<Message> {
    Task::perform(
        async move {
            db.save_template(&template)
                .await
                .map_err(|e| format!("Erro ao salvar modelo: {e}"))
        },
        Message::DbOperationFinished,
    )
}

fn spawn_delete_template(db: std::sync::Arc<crate::core::db::Db>, id: String) -> Task<Message> {
    Task::perform(
        async move {
            db.delete_template(&id)
                .await
                .map_err(|e| format!("Erro ao excluir modelo: {e}"))
        },
        Message::DbOperationFinished,
    )
}

fn spawn_save_dataset(
    db: std::sync::Arc<crate::core::db::Db>,
    id: String,
    name: String,
    table: std::sync::Arc<crate::core::datasets::models::TableData>,
) -> Task<Message> {
    Task::perform(
        async move {
            db.save_dataset(&id, &name, table)
                .await
                .map_err(|e| format!("Erro ao salvar base de dados: {e}"))
        },
        Message::DbOperationFinished,
    )
}

fn spawn_rename_dataset(
    db: std::sync::Arc<crate::core::db::Db>,
    id: String,
    name: String,
) -> Task<Message> {
    Task::perform(
        async move {
            db.rename_dataset(&id, &name)
                .await
                .map_err(|e| format!("Erro ao renomear base de dados: {e}"))
        },
        Message::DbOperationFinished,
    )
}

fn spawn_load_dataset_table(
    db: std::sync::Arc<crate::core::db::Db>,
    generation: u64,
    id: String,
) -> Task<Message> {
    Task::perform(
        async move {
            let result = db
                .load_dataset_table(&id)
                .await
                .map_err(|e| format!("Erro ao carregar base de dados: {e}"));
            (generation, id, result)
        },
        |(generation, id, result)| Message::DatasetTableLoaded {
            generation,
            id,
            result,
        },
    )
}

fn spawn_print_status_auto_dismiss(seq: u64) -> Task<Message> {
    Task::perform(
        async {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        },
        move |_| {
            Message::Controls(crate::core::controls::models::Message::AutoDismissPrintStatus(seq))
        },
    )
}

fn spawn_delete_dataset(db: std::sync::Arc<crate::core::db::Db>, id: String) -> Task<Message> {
    Task::perform(
        async move {
            db.delete_dataset(&id)
                .await
                .map_err(|e| format!("Erro ao excluir base de dados: {e}"))
        },
        Message::DbOperationFinished,
    )
}
