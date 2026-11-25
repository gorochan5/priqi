use iced::Task;

use crate::core::controls::models::PrinterOption;
use crate::core::editor::models::PreviewCard;
use crate::core::models::{Message, State};
use crate::core::printer::PrintDestination;
use crate::core::renderer::LabelRenderOptions;
use crate::core::widgets::import_modal;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ModifiersChanged(modifiers) => {
            state.datasets.shift_pressed = modifiers.shift();
            Task::none()
        }
        Message::WindowResized(id) => iced::window::is_maximized(id)
            .map(|maximized| Message::Navbar(crate::core::navbar::models::Message::SetMaximized(maximized))),
        Message::CloseRequested(id) => {
            let flush_task = flush_pending_edits(state);
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
            Ok((db, templates, loaded_datasets)) => {
                state.db = Some(db);
                state.db_error = None;

                state.templates.templates = templates;
                state.templates.template_counter =
                    max_id_suffix(state.templates.templates.iter().map(|t| t.id.as_str()));
                state.templates.selected_id =
                    state.templates.templates.first().map(|t| t.id.clone());

                state.datasets.options.clear();
                state.datasets.tables.clear();
                for (id, name, table) in loaded_datasets {
                    let opt = crate::core::datasets::models::DatasetOption {
                        id: id.clone(),
                        name,
                    };
                    state.datasets.options.push(opt);
                    state.datasets.tables.insert(id, table);
                }
                state.datasets.dataset_counter =
                    max_id_suffix(state.datasets.options.iter().map(|d| d.id.as_str()));
                state.datasets.dataset = state.datasets.options.first().cloned();
                state.datasets.selected_rows.clear();
                state.datasets.last_selected_row = None;
                if let Some(t) = state.datasets.current_table()
                    && !t.rows.is_empty()
                {
                    state.datasets.selected_rows.insert(0);
                    state.datasets.last_selected_row = Some(0);
                }

                sync_editor_from_selected_template(state);
                sync_can_print(state);

                refresh_all_template_previews(state);
                refresh_preview(state);

                Task::perform(
                    async {
                        tokio::task::spawn_blocking(
                            crate::core::printer::enumerate_windows_printers,
                        )
                        .await
                        .unwrap_or_default()
                    },
                    Message::PrintersLoaded,
                )
            }
            Err(err) => {
                tracing::error!("Falha ao inicializar banco: {err}");
                state.db_error = Some(format!("Erro ao abrir banco de dados: {err}"));
                Task::none()
            }
        },
        Message::Navbar(msg) => {
            if matches!(msg, crate::core::navbar::models::Message::Close) {
                let flush_task = flush_pending_edits(state);
                return flush_task.chain(iced::window::latest().and_then(iced::window::close));
            }
            let blur_task = commit_editor_if_focused(state);
            let nav_task =
                crate::core::navbar::update::update(&mut state.navbar, msg).map(Message::Navbar);
            Task::batch([blur_task, nav_task])
        }
        Message::Controls(msg) => {
            if matches!(msg, crate::core::controls::models::Message::PrintPressed) {
                let flush_task = flush_pending_edits(state);

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
                    let mut selected = Vec::new();
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
            let is_unfocus_or_tab = matches!(
                msg,
                crate::core::editor::models::Message::Unfocus
                    | crate::core::editor::models::Message::SelectTab(_)
            );
            let is_label_config = matches!(
                msg,
                crate::core::editor::models::Message::LabelWidthChanged(_)
                    | crate::core::editor::models::Message::LabelHeightChanged(_)
                    | crate::core::editor::models::Message::LabelDpiSelected(_)
            );

            let task =
                crate::core::editor::update::update(&mut state.editor, msg).map(Message::Editor);

            let mut save_task = Task::none();
            if is_unfocus_or_tab {
                if state.editor.is_dirty {
                    state.editor.is_dirty = false;
                    save_task = sync_active_template_and_previews(state);
                }
            } else if is_label_config
                && let Some(sel_id) = &state.templates.selected_id
                && let Some(tpl) = state
                    .templates
                    .templates
                    .iter_mut()
                    .find(|t| &t.id == sel_id)
            {
                let changed = (tpl.width_mm - state.editor.label_width_mm).abs() > f32::EPSILON
                    || (tpl.height_mm - state.editor.label_height_mm).abs() > f32::EPSILON
                    || tpl.dpi != state.editor.label_dpi;
                if changed {
                    tpl.width_mm = state.editor.label_width_mm;
                    tpl.height_mm = state.editor.label_height_mm;
                    tpl.dpi = state.editor.label_dpi;
                    if let Some(db) = &state.db {
                        save_task = spawn_save_template(db.clone(), tpl.clone());
                    }
                    refresh_all_template_previews(state);
                    refresh_preview(state);
                }
            }

            sync_can_print(state);
            Task::batch([task, save_task])
        }
        Message::Templates(msg) => {
            let blur_task = commit_editor_if_focused(state);
            let prev_selected_id = state.templates.selected_id.clone();
            let delete_id = match &msg {
                crate::core::templates::models::Message::DeleteTemplate(id) => Some(id.clone()),
                _ => None,
            };

            let task = crate::core::templates::update::update(&mut state.templates, msg)
                .map(Message::Templates);

            let dirty_id = state.templates.take_dirty_id();
            let mut db_task = Task::none();
            if let Some(del_id) = &delete_id {
                if let Some(db) = &state.db {
                    db_task = spawn_delete_template(db.clone(), del_id.clone());
                }
            } else if let Some(d_id) = &dirty_id
                && let Some(tpl) = state.templates.templates.iter().find(|t| &t.id == d_id)
                && let Some(db) = &state.db
            {
                db_task = spawn_save_template(db.clone(), tpl.clone());
            }

            let selection_changed = state.templates.selected_id != prev_selected_id;
            if selection_changed {
                sync_editor_from_selected_template(state);
            }

            if delete_id.is_some() || dirty_id.is_some() || selection_changed {
                refresh_all_template_previews(state);
                refresh_preview(state);
            }

            sync_can_print(state);
            Task::batch([blur_task, task, db_task])
        }
        Message::Datasets(msg) => {
            let blur_task = commit_editor_if_focused(state);
            let prev_dataset_id = state.datasets.dataset.as_ref().map(|d| d.id.clone());
            let prev_selected_rows = state.datasets.selected_rows.clone();

            let task = crate::core::datasets::update::update(&mut state.datasets, msg)
                .map(Message::Datasets);

            let dirty_ids = state.datasets.take_dirty_ids();
            let deleted_ids = state.datasets.take_deleted_ids();

            let mut db_tasks = Vec::new();
            if let Some(db) = &state.db {
                for id in &dirty_ids {
                    if let Some(opt) = state.datasets.options.iter().find(|o| &o.id == id)
                        && let Some(table) = state.datasets.tables.get(id)
                    {
                        db_tasks.push(spawn_save_dataset(
                            db.clone(),
                            id.clone(),
                            opt.name.clone(),
                            table.clone(),
                        ));
                    }
                }
                for id in &deleted_ids {
                    db_tasks.push(spawn_delete_dataset(db.clone(), id.clone()));
                }
            }

            let curr_dataset_id = state.datasets.dataset.as_ref().map(|d| d.id.clone());
            let selection_or_data_changed = !dirty_ids.is_empty()
                || !deleted_ids.is_empty()
                || curr_dataset_id != prev_dataset_id
                || state.datasets.selected_rows != prev_selected_rows;

            if selection_or_data_changed {
                refresh_all_template_previews(state);
                refresh_preview(state);
            }

            sync_can_print(state);
            let mut all_tasks = vec![blur_task, task];
            all_tasks.extend(db_tasks);
            Task::batch(all_tasks)
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
    state.controls.can_print = !state.editor.content.text().trim().is_empty();
}

fn current_render_options(state: &State) -> LabelRenderOptions {
    LabelRenderOptions {
        width_mm: state.editor.label_width_mm,
        height_mm: state.editor.label_height_mm,
        dpi: state.editor.label_dpi,
    }
}

fn flush_pending_edits(state: &mut State) -> Task<Message> {
    let mut tasks = Vec::new();

    if state.editor.is_dirty || state.editor.is_focused {
        state.editor.is_focused = false;
        state.editor.is_dirty = false;
        tasks.push(sync_active_template_and_previews(state));
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

        for id in state.datasets.take_dirty_ids() {
            if let Some(opt) = state.datasets.options.iter().find(|o| o.id == id)
                && let Some(table) = state.datasets.tables.get(&id)
            {
                tasks.push(spawn_save_dataset(
                    db.clone(),
                    id,
                    opt.name.clone(),
                    table.clone(),
                ));
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
    if let Some(sel_id) = &state.templates.selected_id {
        let current_text = state.editor.content.text();
        if let Some(tpl) = state
            .templates
            .templates
            .iter_mut()
            .find(|t| &t.id == sel_id)
        {
            tpl.zpl = current_text;
            if let Some(db) = &state.db {
                save_task = spawn_save_template(db.clone(), tpl.clone());
            }
        }
    }

    refresh_all_template_previews(state);
    refresh_preview(state);
    save_task
}

fn refresh_all_template_previews(state: &mut State) {
    let first_row_data: Option<(Vec<String>, Vec<String>)> =
        state.datasets.current_table().and_then(|table| {
            let row_idx = state.datasets.first_selected_row()?;
            table
                .rows
                .get(row_idx)
                .map(|r| (table.headers.clone(), r.clone()))
        });

    for tpl in &mut state.templates.templates {
        if let Some((headers, row)) = &first_row_data {
            let resolved = crate::core::renderer::substitute_template(&tpl.zpl, headers, row);
            tpl.refresh_preview(Some(&resolved));
        } else {
            tpl.refresh_preview(None);
        }
    }
}

fn refresh_preview(state: &mut State) {
    let raw_zpl = state.editor.content.text();
    let render_opts = current_render_options(state);

    if raw_zpl.trim().is_empty() {
        state.editor.preview_cards.clear();
        state.editor.maximized_card = None;
        return;
    }

    let mut cards = Vec::new();

    if let Some(table) = state.datasets.current_table()
        && !state.datasets.selected_rows.is_empty()
    {
        for &row_idx in &state.datasets.selected_rows {
            if let Some(row) = table.rows.get(row_idx) {
                let resolved =
                    crate::core::renderer::substitute_template(&raw_zpl, &table.headers, row);
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

                match crate::core::renderer::render_zpl_all_to_handles(&resolved, render_opts) {
                    Ok(handles) => {
                        let total = handles.len();
                        for (idx, handle) in handles.into_iter().enumerate() {
                            let title = if total > 1 {
                                format!("{base_title} ({}/{total})", idx + 1)
                            } else {
                                base_title.clone()
                            };
                            cards.push(PreviewCard {
                                title,
                                subtitle: subtitle.clone(),
                                image: Some(handle),
                                error: None,
                            });
                        }
                    }
                    Err(err) => cards.push(PreviewCard {
                        title: base_title,
                        subtitle,
                        image: None,
                        error: Some(err),
                    }),
                }
            }
        }
    }

    if cards.is_empty() {
        match crate::core::renderer::render_zpl_all_to_handles(&raw_zpl, render_opts) {
            Ok(handles) => {
                let total = handles.len();
                for (idx, handle) in handles.into_iter().enumerate() {
                    let title = if total > 1 {
                        format!("Visualização do Modelo ({}/{total})", idx + 1)
                    } else {
                        "Visualização do Modelo".to_string()
                    };
                    cards.push(PreviewCard {
                        title,
                        subtitle: String::new(),
                        image: Some(handle),
                        error: None,
                    });
                }
            }
            Err(err) => cards.push(PreviewCard {
                title: "Visualização do Modelo".to_string(),
                subtitle: String::new(),
                image: None,
                error: Some(err),
            }),
        }
    }

    if cards.len() <= 1 {
        state.editor.maximized_card = None;
    } else if let Some(max_idx) = state.editor.maximized_card
        && max_idx >= cards.len()
    {
        state.editor.maximized_card = Some(cards.len() - 1);
    }

    state.editor.preview_cards = cards;
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
    table: crate::core::datasets::models::TableData,
) -> Task<Message> {
    Task::perform(
        async move {
            db.save_dataset(&id, &name, &table)
                .await
                .map_err(|e| format!("Erro ao salvar base de dados: {e}"))
        },
        Message::DbOperationFinished,
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
