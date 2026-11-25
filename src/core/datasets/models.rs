use std::collections::{BTreeSet, HashMap};

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
    ImportModal(import_modal::Message),
}

#[derive(Debug, Default)]
pub struct State {
    pub options: Vec<DatasetOption>,
    pub dataset: Option<DatasetOption>,
    pub tables: HashMap<String, TableData>,
    pub selected_rows: BTreeSet<usize>,
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
    pub import_modal: import_modal::State,
}

impl State {
    pub fn mark_dirty(&mut self, id: &str) {
        if !self.dirty_dataset_ids.iter().any(|existing| existing == id) {
            self.dirty_dataset_ids.push(id.to_string());
        }
    }

    pub fn take_dirty_ids(&mut self) -> Vec<String> {
        std::mem::take(&mut self.dirty_dataset_ids)
    }

    pub fn take_deleted_ids(&mut self) -> Vec<String> {
        std::mem::take(&mut self.deleted_dataset_ids)
    }

    pub fn current_table(&self) -> Option<&TableData> {
        self.dataset.as_ref().and_then(|d| self.tables.get(&d.id))
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
