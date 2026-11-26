use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use crate::core::widgets::import_modal;

pub const TABLE_SCROLLABLE_ID: &str = "datasets_table_scrollable";
pub const TABLE_ROWS_SCROLLABLE_ID: &str = "datasets_table_rows_scrollable";
pub const CELL_INPUT_ID: &str = "datasets_cell_input";
pub const DATASET_RENAME_INPUT_ID: &str = "datasets_rename_input";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatasetOption {
    pub id: String,
    pub name: String,
}

impl std::fmt::Display for DatasetOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn add_column(&mut self, header_name: String) {
        self.headers.push(header_name);
        for row in &mut self.rows {
            row.push(String::new());
        }
    }

    pub fn remove_column(&mut self, col: usize) {
        if col < self.headers.len() {
            self.headers.remove(col);
            for row in &mut self.rows {
                if col < row.len() {
                    row.remove(col);
                }
            }
        }
    }

    pub fn add_row(&mut self) {
        let col_count = self.headers.len();
        self.rows.push(vec![String::new(); col_count]);
    }

    pub fn remove_rows(&mut self, rows_to_remove: &BTreeSet<usize>) {
        if rows_to_remove.is_empty() {
            return;
        }
        let mut idx = 0;
        self.rows.retain(|_| {
            let keep = !rows_to_remove.contains(&idx);
            idx += 1;
            keep
        });
    }

    pub fn set_cell(&mut self, row: usize, col: usize, val: String) {
        if let Some(r) = self.rows.get_mut(row)
            && let Some(c) = r.get_mut(col)
        {
            *c = val;
        }
    }

    pub fn set_header(&mut self, col: usize, val: String) {
        if let Some(h) = self.headers.get_mut(col) {
            *h = val;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellCoord {
    Header(usize),
    Cell { row: usize, col: usize },
}

#[derive(Debug, Clone)]
pub enum Message {
    DatasetSelected(DatasetOption),
    ToggleDatasetDropdown,
    CloseDatasetDropdown,
    AddDataset,
    StartRenameDataset(String),
    DatasetNameChanged(String),
    SaveDatasetRename,
    CancelDatasetRename,
    DeleteDataset(String),
    SelectRow(usize),
    ToggleRowSelection(usize),
    SelectAllRows,
    ClearSelection,
    DeleteSelectedRows,
    StartEditingCell(CellCoord),
    EditingValueChanged(String),
    SaveCell,
    CancelEditing,
    AddColumn,
    DeleteColumn(usize),
    AddRow,
    SearchChanged(String),
    TableScrolled { y: f32, height: f32 },
    CsvImportFinished(Result<(String, Arc<TableData>), String>),
    ImportModal(import_modal::Message),
}

#[derive(Debug)]
pub struct State {
    pub options: Vec<DatasetOption>,
    pub dataset: Option<DatasetOption>,
    pub tables: HashMap<String, Arc<TableData>>,
    pub selected_rows: BTreeSet<usize>,
    pub selection_version: u64,
    pub last_selected_row: Option<usize>,
    pub shift_pressed: bool,
    pub editing_cell: Option<CellCoord>,
    pub editing_value: String,
    pub is_dropdown_open: bool,
    pub renaming_dataset_id: Option<String>,
    pub dataset_name_input: String,
    pub search_query: String,
    pub dataset_counter: usize,
    pub dirty_dataset_ids: Vec<String>,
    pub deleted_dataset_ids: Vec<String>,
    pub renamed_datasets: Vec<(String, String)>,
    pub scroll_y: f32,
    pub viewport_height: f32,
    pub cached_filtered_indices: Option<Vec<usize>>,
    pub is_loading_dataset: bool,
    pub import_modal: import_modal::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            dataset: None,
            tables: HashMap::new(),
            selected_rows: BTreeSet::new(),
            selection_version: 0,
            last_selected_row: None,
            shift_pressed: false,
            editing_cell: None,
            editing_value: String::new(),
            is_dropdown_open: false,
            renaming_dataset_id: None,
            dataset_name_input: String::new(),
            search_query: String::new(),
            dataset_counter: 0,
            dirty_dataset_ids: Vec::new(),
            deleted_dataset_ids: Vec::new(),
            renamed_datasets: Vec::new(),
            scroll_y: 0.0,
            viewport_height: 600.0,
            cached_filtered_indices: None,
            is_loading_dataset: false,
            import_modal: import_modal::State::default(),
        }
    }
}

impl State {
    pub fn bump_selection_version(&mut self) {
        self.selection_version = self.selection_version.wrapping_add(1);
    }

    pub fn mark_dirty(&mut self, id: &str) {
        if !self.dirty_dataset_ids.iter().any(|existing| existing == id) {
            self.dirty_dataset_ids.push(id.to_string());
        }
    }

    pub fn mark_renamed(&mut self, id: &str, name: String) {
        if let Some(entry) = self
            .renamed_datasets
            .iter_mut()
            .find(|(existing_id, _)| existing_id == id)
        {
            entry.1 = name;
        } else {
            self.renamed_datasets.push((id.to_string(), name));
        }
    }

    pub fn take_dirty_ids(&mut self) -> Vec<String> {
        std::mem::take(&mut self.dirty_dataset_ids)
    }

    pub fn take_deleted_ids(&mut self) -> Vec<String> {
        std::mem::take(&mut self.deleted_dataset_ids)
    }

    pub fn take_renamed_datasets(&mut self) -> Vec<(String, String)> {
        std::mem::take(&mut self.renamed_datasets)
    }

    pub fn current_table(&self) -> Option<&TableData> {
        self.dataset
            .as_ref()
            .and_then(|d| self.tables.get(&d.id))
            .map(Arc::as_ref)
    }

    pub fn rebuild_filtered_indices(&mut self) {
        if self.search_query.trim().is_empty() {
            self.cached_filtered_indices = None;
            return;
        }
        let Some(table) = self.current_table() else {
            self.cached_filtered_indices = None;
            return;
        };
        let indices =
            crate::core::datasets::update::filtered_row_indices(table, &self.search_query);
        self.cached_filtered_indices = Some(indices);
    }

    pub fn visible_row_count(&self) -> usize {
        match &self.cached_filtered_indices {
            Some(v) => v.len(),
            None => self.current_table().map_or(0, |t| t.rows.len()),
        }
    }

    pub fn row_index_at(&self, visible_pos: usize) -> Option<usize> {
        match &self.cached_filtered_indices {
            Some(v) => v.get(visible_pos).copied(),
            None => self
                .current_table()
                .and_then(|t| (visible_pos < t.rows.len()).then_some(visible_pos)),
        }
    }

    pub fn selection_flags(&self) -> (bool, bool) {
        if let Some(indices) = &self.cached_filtered_indices {
            if indices.is_empty() || self.selected_rows.is_empty() {
                return (false, false);
            }
            let mut some = false;
            let mut all = true;
            for idx in indices {
                if self.selected_rows.contains(idx) {
                    some = true;
                } else {
                    all = false;
                }
                if some && !all {
                    break;
                }
            }
            (all, some)
        } else {
            let total = self.current_table().map_or(0, |t| t.rows.len());
            if total == 0 || self.selected_rows.is_empty() {
                (false, false)
            } else {
                (self.selected_rows.len() >= total, true)
            }
        }
    }

    pub fn evict_inactive_tables(&mut self) {
        let active_id = self.dataset.as_ref().map(|d| d.id.as_str());
        let dirty = &self.dirty_dataset_ids;
        self.tables
            .retain(|id, _| active_id == Some(id.as_str()) || dirty.iter().any(|d| d == id));
    }

    pub fn is_row_selected(&self, row: usize) -> bool {
        self.selected_rows.contains(&row)
    }

    pub fn selected_count(&self) -> usize {
        self.selected_rows.len()
    }

    pub fn first_selected_row(&self) -> Option<usize> {
        self.selected_rows.iter().next().copied()
    }

    pub fn search_placeholder(&self) -> &'static str {
        "Buscar no dataset..."
    }
}
