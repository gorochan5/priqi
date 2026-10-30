use std::collections::HashMap;

use crate::core::widgets::import_modal;

pub const TABLE_SCROLLABLE_ID: &str = "datasets_table_scrollable";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Dataset {
    #[default]
    LocaisDeEstoque,
    Usuarios,
}

impl Dataset {
    pub const ALL: [Dataset; 2] = [Dataset::LocaisDeEstoque, Dataset::Usuarios];
}

impl std::fmt::Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocaisDeEstoque => write!(f, "Locais de Estoque"),
            Self::Usuarios => write!(f, "Usuários"),
        }
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

    pub fn add_row(&mut self) {
        let col_count = self.headers.len();
        self.rows.push(vec![String::new(); col_count]);
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
    Nothing,
    DatasetSelected(Dataset),
    SelectRow(usize),
    StartEditingCell(CellCoord),
    EditingValueChanged(String),
    SaveCell,
    CancelEditing,
    AddColumn,
    AddRow,
    SearchChanged(String),
    ImportModal(import_modal::Message),
}

#[derive(Debug)]
pub struct State {
    pub dataset: Option<Dataset>,
    pub tables: HashMap<Dataset, TableData>,
    pub selected_row: Option<usize>,
    pub editing_cell: Option<CellCoord>,
    pub editing_value: String,
    pub search_query: String,
    pub column_counter: usize,
    pub import_modal: import_modal::State,
}

impl Default for State {
    fn default() -> Self {
        let mut tables = HashMap::new();

        let locais = TableData {
            headers: vec![
                "LOCATION / BIN".to_string(),
                "SKU IDENTIFIER".to_string(),
            ],
            rows: vec![
                vec!["Aisle-01-Shelf-B".to_string(), "SKU-10234".to_string()],
                vec!["Aisle-02-Shelf-A".to_string(), "SKU-88213".to_string()],
                vec!["Recv-Dock-01".to_string(), "SKU-S5501".to_string()],
                vec!["Aisle-07-Bin-C".to_string(), "SKU-44108".to_string()],
                vec!["Cold-Room-02".to_string(), "SKU-71006".to_string()],
            ],
        };

        let usuarios = TableData {
            headers: vec![
                "USUÁRIO".to_string(),
                "MATRÍCULA".to_string(),
                "DEPARTAMENTO".to_string(),
            ],
            rows: vec![
                vec![
                    "Carlos Silva".to_string(),
                    "USR-0042".to_string(),
                    "Expedição".to_string(),
                ],
                vec![
                    "Mariana Santos".to_string(),
                    "USR-0089".to_string(),
                    "Almoxarifado".to_string(),
                ],
                vec![
                    "Roberto Almeida".to_string(),
                    "USR-0112".to_string(),
                    "Produção".to_string(),
                ],
                vec![
                    "Ana Paula Costa".to_string(),
                    "USR-0158".to_string(),
                    "Controle de Qualidade".to_string(),
                ],
                vec![
                    "Lucas Fernandes".to_string(),
                    "USR-0203".to_string(),
                    "Logística".to_string(),
                ],
            ],
        };

        tables.insert(Dataset::LocaisDeEstoque, locais);
        tables.insert(Dataset::Usuarios, usuarios);

        Self {
            dataset: Some(Dataset::LocaisDeEstoque),
            tables,
            selected_row: Some(1),
            editing_cell: None,
            editing_value: String::new(),
            search_query: String::new(),
            column_counter: 0,
            import_modal: import_modal::State::default(),
        }
    }
}

impl State {
    pub fn current_table(&self) -> Option<&TableData> {
        self.dataset.and_then(|d| self.tables.get(&d))
    }

    pub fn current_table_mut(&mut self) -> Option<&mut TableData> {
        let d = self.dataset?;
        self.tables.get_mut(&d)
    }

    pub fn search_placeholder(&self) -> &'static str {
        match self.dataset {
            Some(Dataset::LocaisDeEstoque) => "Search location, SKU or batch...",
            Some(Dataset::Usuarios) => "Buscar usuário, matrícula ou setor...",
            None => "Buscar no dataset...",
        }
    }
}
