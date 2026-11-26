use std::sync::Arc;

use iced::Task;
use iced::widget::operation::Animation;
use iced::widget::scrollable::RelativeOffset;

use crate::core::datasets::models::{
    CELL_INPUT_ID, CellCoord, DATASET_RENAME_INPUT_ID, DatasetOption, Message, State,
    TABLE_ROWS_SCROLLABLE_ID, TABLE_SCROLLABLE_ID, TableData,
};
use crate::core::widgets::import_modal::{self, models::ImportTab};

const MAX_CSV_IMPORT_BYTES: usize = 25 * 1024 * 1024;

fn snap_table_rows_to_top() -> Task<Message> {
    iced::widget::operation::scrollable::snap_to(
        TABLE_ROWS_SCROLLABLE_ID,
        RelativeOffset {
            x: None,
            y: Some(0.0),
        },
        Animation::Instant,
    )
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::DatasetSelected(dataset) => {
            save_current_edit(state);
            clear_dataset_rename(state);
            state.is_dropdown_open = false;
            state.dataset = Some(dataset);
            state.selected_rows.clear();
            state.last_selected_row = None;
            if let Some(t) = state.current_table()
                && !t.rows.is_empty()
            {
                state.selected_rows.insert(0);
                state.last_selected_row = Some(0);
            }
            state.editing_cell = None;
            state.editing_value.clear();
            state.search_query.clear();
            state.scroll_y = 0.0;
            state.rebuild_filtered_indices();
            state.bump_selection_version();
            snap_table_rows_to_top()
        }
        Message::ToggleDatasetDropdown => {
            state.is_dropdown_open = !state.is_dropdown_open;
            Task::none()
        }
        Message::CloseDatasetDropdown => {
            state.is_dropdown_open = false;
            Task::none()
        }
        Message::AddDataset => {
            save_current_edit(state);
            clear_dataset_rename(state);
            state.is_dropdown_open = false;
            let new_id = next_unique_dataset_id(state, "dataset");
            let opt = DatasetOption {
                id: new_id.clone(),
                name: format!("Dataset {}", state.dataset_counter),
            };
            let table = Arc::new(TableData {
                headers: vec!["COLUNA 1".to_string(), "COLUNA 2".to_string()],
                rows: vec![vec![String::new(), String::new()]],
            });
            state.tables.insert(opt.id.clone(), table);
            state.options.push(opt.clone());
            state.dataset = Some(opt);
            state.selected_rows.clear();
            state.selected_rows.insert(0);
            state.last_selected_row = Some(0);
            state.editing_cell = None;
            state.editing_value.clear();
            state.search_query.clear();
            state.scroll_y = 0.0;
            state.mark_dirty(&new_id);
            state.rebuild_filtered_indices();
            state.bump_selection_version();
            snap_table_rows_to_top()
        }
        Message::StartRenameDataset(id) => {
            save_current_edit(state);
            state.is_dropdown_open = false;
            if let Some(opt) = state.options.iter().find(|o| o.id == id) {
                state.renaming_dataset_id = Some(id);
                state.dataset_name_input = opt.name.clone();
                return iced::widget::operation::focus(DATASET_RENAME_INPUT_ID);
            }
            Task::none()
        }
        Message::DatasetNameChanged(val) => {
            state.dataset_name_input = val;
            Task::none()
        }
        Message::SaveDatasetRename => {
            let trimmed = state.dataset_name_input.trim().to_string();
            if !trimmed.is_empty()
                && let Some(id) = state.renaming_dataset_id.as_ref()
            {
                let id = id.clone();
                let mut changed = false;
                if let Some(opt) = state.options.iter_mut().find(|o| o.id == id)
                    && opt.name != trimmed
                {
                    opt.name = trimmed.clone();
                    changed = true;
                }
                if let Some(curr) = &mut state.dataset
                    && curr.id == id
                    && curr.name != trimmed
                {
                    curr.name = trimmed.clone();
                    changed = true;
                }
                if changed {
                    state.mark_renamed(&id, trimmed);
                }
            }
            clear_dataset_rename(state);
            Task::none()
        }
        Message::CancelDatasetRename => {
            clear_dataset_rename(state);
            Task::none()
        }
        Message::DeleteDataset(id) => {
            state.editing_cell = None;
            state.editing_value.clear();
            clear_dataset_rename(state);

            let mut active_changed = false;
            if state.options.iter().any(|o| o.id == id) {
                state.tables.remove(&id);
                state.options.retain(|o| o.id != id);
                state.dirty_dataset_ids.retain(|dirty_id| dirty_id != &id);
                state
                    .renamed_datasets
                    .retain(|(renamed_id, _)| renamed_id != &id);
                if !state.deleted_dataset_ids.contains(&id) {
                    state.deleted_dataset_ids.push(id.clone());
                }

                if state.dataset.as_ref().is_some_and(|d| d.id == id) {
                    active_changed = true;
                    state.dataset = state.options.first().cloned();
                    state.selected_rows.clear();
                    state.last_selected_row = None;
                    state.search_query.clear();
                    state.scroll_y = 0.0;
                    if let Some(t) = state.current_table()
                        && !t.rows.is_empty()
                    {
                        state.selected_rows.insert(0);
                        state.last_selected_row = Some(0);
                    }
                    state.rebuild_filtered_indices();
                    state.bump_selection_version();
                }
            }
            if state.options.is_empty() {
                state.is_dropdown_open = false;
            }
            if active_changed {
                snap_table_rows_to_top()
            } else {
                Task::none()
            }
        }
        Message::SelectRow(row_idx) => {
            save_current_edit(state);
            let changed = if state.shift_pressed
                && let Some(anchor) = state.last_selected_row
            {
                select_row_range(state, anchor, row_idx)
            } else if state.selected_rows.len() == 1 && state.selected_rows.contains(&row_idx) {
                false
            } else {
                state.selected_rows.clear();
                state.selected_rows.insert(row_idx);
                true
            };
            if changed {
                state.bump_selection_version();
            }
            state.last_selected_row = Some(row_idx);
            Task::none()
        }
        Message::ToggleRowSelection(row_idx) => {
            save_current_edit(state);
            let changed = if state.shift_pressed
                && let Some(anchor) = state.last_selected_row
            {
                select_row_range(state, anchor, row_idx)
            } else if state.selected_rows.contains(&row_idx) {
                state.selected_rows.remove(&row_idx)
            } else {
                state.selected_rows.insert(row_idx)
            };
            if changed {
                state.bump_selection_version();
            }
            state.last_selected_row = Some(row_idx);
            Task::none()
        }
        Message::SelectAllRows => {
            save_current_edit(state);
            if let Some(matching) = state.cached_filtered_indices.as_ref() {
                if !matching.is_empty() {
                    let all_matching_selected =
                        matching.iter().all(|idx| state.selected_rows.contains(idx));
                    let mut changed = false;
                    if all_matching_selected {
                        for idx in matching {
                            if state.selected_rows.remove(idx) {
                                changed = true;
                            }
                        }
                        state.last_selected_row = None;
                    } else {
                        for &idx in matching {
                            if state.selected_rows.insert(idx) {
                                changed = true;
                            }
                        }
                        state.last_selected_row = matching.last().copied();
                    }
                    if changed {
                        state.bump_selection_version();
                    }
                }
            } else if let Some(table) = state.current_table() {
                let total = table.rows.len();
                if total > 0 {
                    let mut changed = false;
                    if state.selected_rows.len() >= total {
                        if !state.selected_rows.is_empty() {
                            state.selected_rows.clear();
                            changed = true;
                        }
                        state.last_selected_row = None;
                    } else {
                        for idx in 0..total {
                            if state.selected_rows.insert(idx) {
                                changed = true;
                            }
                        }
                        state.last_selected_row = Some(total - 1);
                    }
                    if changed {
                        state.bump_selection_version();
                    }
                }
            }
            Task::none()
        }
        Message::ClearSelection => {
            save_current_edit(state);
            if !state.selected_rows.is_empty() {
                state.selected_rows.clear();
                state.bump_selection_version();
            }
            state.last_selected_row = None;
            Task::none()
        }
        Message::DeleteSelectedRows => {
            save_current_edit(state);
            if !state.selected_rows.is_empty()
                && let Some(curr_id) = state.dataset.as_ref().map(|d| d.id.clone())
                && let Some(table_arc) = state.tables.get_mut(&curr_id)
            {
                let table = Arc::make_mut(table_arc);
                let to_remove = std::mem::take(&mut state.selected_rows);
                table.remove_rows(&to_remove);
                state.last_selected_row = None;
                if !table.rows.is_empty() {
                    state.selected_rows.insert(0);
                    state.last_selected_row = Some(0);
                }
                state.mark_dirty(&curr_id);
                state.rebuild_filtered_indices();
                state.bump_selection_version();
            }
            Task::none()
        }
        Message::StartEditingCell(coord) => {
            save_current_edit(state);
            let current_val = match coord {
                CellCoord::Header(col) => state
                    .current_table()
                    .and_then(|t| t.headers.get(col))
                    .cloned()
                    .unwrap_or_default(),
                CellCoord::Cell { row, col } => state
                    .current_table()
                    .and_then(|t| t.rows.get(row))
                    .and_then(|r| r.get(col))
                    .cloned()
                    .unwrap_or_default(),
            };
            state.editing_cell = Some(coord);
            state.editing_value = current_val;
            iced::widget::operation::focus(CELL_INPUT_ID)
        }
        Message::EditingValueChanged(val) => {
            state.editing_value = val;
            Task::none()
        }
        Message::SaveCell => {
            save_current_edit(state);
            Task::none()
        }
        Message::CancelEditing => {
            state.editing_cell = None;
            state.editing_value.clear();
            Task::none()
        }
        Message::AddColumn => {
            save_current_edit(state);
            if let Some(curr_id) = state.dataset.as_ref().map(|d| d.id.clone())
                && let Some(table_arc) = state.tables.get_mut(&curr_id)
            {
                let table = Arc::make_mut(table_arc);
                let col_name = next_unique_column_name(&table.headers);
                table.add_column(col_name);
                state.mark_dirty(&curr_id);
                if !state.selected_rows.is_empty() {
                    state.bump_selection_version();
                }
            }
            iced::widget::operation::scrollable::snap_to(
                TABLE_SCROLLABLE_ID,
                RelativeOffset {
                    x: Some(1.0),
                    y: None,
                },
                Animation::Instant,
            )
        }
        Message::DeleteColumn(col_idx) => {
            save_current_edit(state);
            if let Some(curr_id) = state.dataset.as_ref().map(|d| d.id.clone())
                && let Some(table_arc) = state.tables.get_mut(&curr_id)
                && col_idx < table_arc.headers.len()
            {
                Arc::make_mut(table_arc).remove_column(col_idx);
                state.mark_dirty(&curr_id);
                state.rebuild_filtered_indices();
                if !state.selected_rows.is_empty() {
                    state.bump_selection_version();
                }
            }
            Task::none()
        }
        Message::AddRow => {
            save_current_edit(state);
            if let Some(curr_id) = state.dataset.as_ref().map(|d| d.id.clone())
                && let Some(table_arc) = state.tables.get_mut(&curr_id)
            {
                let table = Arc::make_mut(table_arc);
                table.add_row();
                let last_idx = table.rows.len() - 1;
                state.selected_rows.clear();
                state.selected_rows.insert(last_idx);
                state.last_selected_row = Some(last_idx);
                state.mark_dirty(&curr_id);
                state.rebuild_filtered_indices();
                state.bump_selection_version();
            }
            iced::widget::operation::scrollable::snap_to(
                TABLE_ROWS_SCROLLABLE_ID,
                RelativeOffset {
                    x: None,
                    y: Some(1.0),
                },
                Animation::Instant,
            )
        }
        Message::SearchChanged(query) => {
            state.search_query = query;
            state.scroll_y = 0.0;
            state.rebuild_filtered_indices();
            snap_table_rows_to_top()
        }
        Message::TableScrolled { y, height } => {
            state.scroll_y = y.max(0.0);
            if height > 0.0 {
                state.viewport_height = height;
            }
            Task::none()
        }
        Message::CsvImportFinished(result) => {
            state.import_modal.is_importing = false;
            match result {
                Ok((suggested_name, table_arc)) => {
                    save_current_edit(state);
                    clear_dataset_rename(state);
                    state.is_dropdown_open = false;
                    let new_id = next_unique_dataset_id(state, "dataset_import");
                    let name = if suggested_name.trim().is_empty() {
                        format!("Dataset Importado {}", state.dataset_counter)
                    } else {
                        suggested_name
                    };

                    let opt = DatasetOption {
                        id: new_id.clone(),
                        name,
                    };
                    state.tables.insert(opt.id.clone(), table_arc);
                    state.options.push(opt.clone());
                    state.dataset = Some(opt);
                    state.mark_dirty(&new_id);
                    state.evict_inactive_tables();

                    state.editing_cell = None;
                    state.editing_value.clear();
                    state.search_query.clear();
                    state.scroll_y = 0.0;
                    state.selected_rows.clear();
                    if state
                        .current_table()
                        .is_some_and(|t| !t.rows.is_empty())
                    {
                        state.selected_rows.insert(0);
                        state.last_selected_row = Some(0);
                    } else {
                        state.last_selected_row = None;
                    }
                    state.rebuild_filtered_indices();
                    state.bump_selection_version();
                    state.import_modal.reset();
                    snap_table_rows_to_top()
                }
                Err(err) => {
                    state.import_modal.file_error = Some(err);
                    Task::none()
                }
            }
        }
        Message::ImportModal(msg) => {
            state.import_modal.kind = Some(import_modal::models::ImportKind::Datasets);
            if matches!(msg, import_modal::Message::Open) {
                clear_dataset_rename(state);
                state.is_dropdown_open = false;
            }
            if matches!(msg, import_modal::Message::ImportPressed) {
                if state.import_modal.is_importing {
                    return Task::none();
                }

                if state.import_modal.active_tab == ImportTab::File {
                    let Some(path) = state.import_modal.selected_file.clone() else {
                        state.import_modal.file_error =
                            Some("Selecione um arquivo CSV antes de importar".to_string());
                        return Task::none();
                    };

                    let suggested_name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();

                    state.import_modal.is_importing = true;
                    state.import_modal.file_error = None;

                    return Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                let meta = std::fs::metadata(&path)
                                    .map_err(|e| format!("Erro ao acessar arquivo: {e}"))?;
                                if meta.len() > MAX_CSV_IMPORT_BYTES as u64 {
                                    return Err("Arquivo excede o limite de 25 MB".to_string());
                                }
                                let bytes = std::fs::read(&path)
                                    .map_err(|e| format!("Erro ao ler arquivo: {e}"))?;
                                if bytes.len() > MAX_CSV_IMPORT_BYTES {
                                    return Err("Arquivo excede o limite de 25 MB".to_string());
                                }
                                let raw = String::from_utf8(bytes)
                                    .map_err(|_| "Arquivo não está em UTF-8 válido".to_string())?;
                                let csv_str = raw.strip_prefix('\u{FEFF}').unwrap_or(&raw);
                                let table = parse_csv_to_table(csv_str)
                                    .map_err(|e| format!("Erro ao interpretar CSV: {e}"))?;
                                Ok((suggested_name, Arc::new(table)))
                            })
                            .await
                            .unwrap_or_else(|e| {
                                Err(format!("Erro na tarefa de importação: {e}"))
                            })
                        },
                        Message::CsvImportFinished,
                    );
                }

                let csv_text = state.import_modal.content.text();
                if csv_text.trim().is_empty() {
                    state.import_modal.file_error = Some(
                        "Cole os dados CSV ou selecione um arquivo antes de importar".to_string(),
                    );
                    return Task::none();
                }

                state.import_modal.is_importing = true;
                state.import_modal.file_error = None;

                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let table = parse_csv_to_table(&csv_text)
                                .map_err(|e| format!("Erro ao interpretar CSV: {e}"))?;
                            Ok((String::new(), Arc::new(table)))
                        })
                        .await
                        .unwrap_or_else(|e| Err(format!("Erro na tarefa de importação: {e}")))
                    },
                    Message::CsvImportFinished,
                )
            } else {
                import_modal::update(&mut state.import_modal, msg).map(Message::ImportModal)
            }
        }
    }
}

fn contains_ascii_case_insensitive(haystack: &str, needle_lower: &[u8]) -> bool {
    let h = haystack.as_bytes();
    let n_len = needle_lower.len();
    if n_len == 0 {
        return true;
    }
    if h.len() < n_len {
        return false;
    }
    h.windows(n_len).any(|w| {
        w.iter()
            .zip(needle_lower)
            .all(|(&hb, &nb)| hb.to_ascii_lowercase() == nb)
    })
}

fn row_matches_query(row: &[String], query_lower: &str, query_is_ascii: bool) -> bool {
    if query_is_ascii {
        let needle = query_lower.as_bytes();
        row.iter()
            .any(|cell| contains_ascii_case_insensitive(cell, needle))
    } else {
        row.iter()
            .any(|cell| !cell.is_ascii() && cell.to_lowercase().contains(query_lower))
    }
}

pub fn filtered_row_indices(table: &TableData, search_query: &str) -> Vec<usize> {
    let trimmed = search_query.trim();
    if trimmed.is_empty() {
        return (0..table.rows.len()).collect();
    }

    let query_lower = trimmed.to_lowercase();
    let query_is_ascii = query_lower.is_ascii();
    table
        .rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row_matches_query(row, &query_lower, query_is_ascii))
        .map(|(idx, _)| idx)
        .collect()
}

fn clear_dataset_rename(state: &mut State) {
    state.renaming_dataset_id = None;
    state.dataset_name_input.clear();
}

fn next_unique_dataset_id(state: &mut State, prefix: &str) -> String {
    loop {
        state.dataset_counter += 1;
        let candidate = format!("{prefix}_{}", state.dataset_counter);
        if !state.options.iter().any(|o| o.id == candidate)
            && !state.tables.contains_key(&candidate)
        {
            return candidate;
        }
    }
}

fn next_unique_column_name(headers: &[String]) -> String {
    let mut n = 1usize;
    loop {
        let candidate = format!("COLUNA {n}");
        if !headers.iter().any(|h| h == &candidate) {
            return candidate;
        }
        n += 1;
    }
}

fn select_row_range(state: &mut State, anchor: usize, target: usize) -> bool {
    let start = anchor.min(target);
    let end = anchor.max(target);
    let trimmed = state.search_query.trim();
    let table_opt = state.dataset.as_ref().and_then(|d| state.tables.get(&d.id));
    let mut changed = false;

    if let Some(table) = table_opt {
        if trimmed.is_empty() {
            let max_end = end.min(table.rows.len().saturating_sub(1));
            if !table.rows.is_empty() && start <= max_end {
                for idx in start..=max_end {
                    if state.selected_rows.insert(idx) {
                        changed = true;
                    }
                }
            }
        } else {
            let query_lower = trimmed.to_lowercase();
            let query_is_ascii = query_lower.is_ascii();
            for idx in start..=end {
                if let Some(row) = table.rows.get(idx)
                    && row_matches_query(row, &query_lower, query_is_ascii)
                    && state.selected_rows.insert(idx)
                {
                    changed = true;
                }
            }
        }
    } else {
        for idx in start..=end {
            if state.selected_rows.insert(idx) {
                changed = true;
            }
        }
    }

    changed
}

fn detect_csv_delimiter(csv_content: &str) -> u8 {
    let first_line = csv_content
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");

    let mut comma_count = 0usize;
    let mut semi_count = 0usize;
    let mut tab_count = 0usize;
    let mut in_quotes = false;
    let bytes = first_line.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                if in_quotes && i + 1 < bytes.len() && bytes[i + 1] == b'"' {
                    i += 2;
                    continue;
                }
                in_quotes = !in_quotes;
            }
            b',' if !in_quotes => comma_count += 1,
            b';' if !in_quotes => semi_count += 1,
            b'\t' if !in_quotes => tab_count += 1,
            _ => {}
        }
        i += 1;
    }

    if semi_count > comma_count && semi_count >= tab_count {
        b';'
    } else if tab_count > comma_count && tab_count > semi_count {
        b'\t'
    } else {
        b','
    }
}

pub fn parse_csv_to_table(csv_content: &str) -> Result<TableData, anyhow::Error> {
    if csv_content.trim().is_empty() {
        anyhow::bail!("CSV vazio ou sem colunas");
    }

    let delimiter = detect_csv_delimiter(csv_content);
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_content.as_bytes());

    let mut headers: Vec<String> = reader
        .headers()?
        .iter()
        .map(|h| h.trim().to_string())
        .collect();

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        rows.push(record.iter().map(String::from).collect::<Vec<String>>());
    }

    let max_cols = headers
        .len()
        .max(rows.iter().map(|r| r.len()).max().unwrap_or(0));

    if max_cols == 0 {
        anyhow::bail!("CSV vazio ou sem colunas");
    }

    while headers.len() < max_cols {
        headers.push(next_unique_column_name(&headers));
    }

    for row in &mut rows {
        while row.len() < max_cols {
            row.push(String::new());
        }
    }

    Ok(TableData { headers, rows })
}

fn save_current_edit(state: &mut State) {
    if let Some(coord) = state.editing_cell.take() {
        let val = std::mem::take(&mut state.editing_value);
        if let Some(curr_id) = state.dataset.as_ref().map(|d| d.id.clone())
            && let Some(table_arc) = state.tables.get_mut(&curr_id)
        {
            match coord {
                CellCoord::Header(col) => {
                    let trimmed = val.trim();
                    if !trimmed.is_empty()
                        && table_arc.headers.get(col).map(String::as_str) != Some(trimmed)
                    {
                        Arc::make_mut(table_arc).set_header(col, trimmed.to_string());
                        state.mark_dirty(&curr_id);
                        if !state.selected_rows.is_empty() {
                            state.bump_selection_version();
                        }
                    }
                }
                CellCoord::Cell { row, col } => {
                    if table_arc
                        .rows
                        .get(row)
                        .and_then(|r| r.get(col))
                        .map(String::as_str)
                        != Some(val.as_str())
                    {
                        Arc::make_mut(table_arc).set_cell(row, col, val);
                        state.mark_dirty(&curr_id);
                        state.rebuild_filtered_indices();
                        if state.selected_rows.contains(&row) {
                            state.bump_selection_version();
                        }
                    }
                }
            }
        }
    } else {
        state.editing_value.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::parse_csv_to_table;

    #[test]
    fn parse_comma_delimited_csv() {
        let csv = " Nome , Idade , Cidade \nAna,30,SP\nBruno,25,RJ\n";
        let table = parse_csv_to_table(csv).expect("should parse comma CSV");
        assert_eq!(table.headers, vec!["Nome", "Idade", "Cidade"]);
        assert_eq!(
            table.rows,
            vec![
                vec!["Ana", "30", "SP"],
                vec!["Bruno", "25", "RJ"],
            ]
        );
    }

    #[test]
    fn parse_semicolon_delimited_csv() {
        let csv = "SKU;Descrição;Preço\n001;\"Item A;B\";10.50\n002;Item C;20.00\n";
        let table = parse_csv_to_table(csv).expect("should parse semicolon CSV");
        assert_eq!(table.headers, vec!["SKU", "Descrição", "Preço"]);
        assert_eq!(
            table.rows,
            vec![
                vec!["001", "Item A;B", "10.50"],
                vec!["002", "Item C", "20.00"],
            ]
        );
    }

    #[test]
    fn parse_ragged_wide_and_short_rows() {
        let csv = "A,B\n1\n2,3,4,5\n";
        let table = parse_csv_to_table(csv).expect("should pad headers and rows");
        assert_eq!(table.headers, vec!["A", "B", "COLUNA 1", "COLUNA 2"]);
        assert_eq!(
            table.rows,
            vec![
                vec!["1", "", "", ""],
                vec!["2", "3", "4", "5"],
            ]
        );
    }

    #[test]
    fn reject_empty_csv() {
        assert!(parse_csv_to_table("   \n  ").is_err());
    }
}
