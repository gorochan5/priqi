use std::path::PathBuf;
use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImportTab {
    #[default]
    Paste,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    Templates,
    Datasets,
}

#[derive(Debug, Clone, Copy)]
pub struct ImportConfig {
    pub title: &'static str,
    pub placeholder: &'static str,
    pub dropzone_hint: &'static str,
    pub extensions: &'static [&'static str],
    pub kind: ImportKind,
}

impl ImportConfig {
    pub const fn for_kind(kind: ImportKind) -> Self {
        match kind {
            ImportKind::Templates => Self::templates(),
            ImportKind::Datasets => Self::datasets(),
        }
    }

    pub const fn templates() -> Self {
        Self {
            title: "Importar Etiqueta",
            placeholder: "Cole seu código ZPL aqui...",
            dropzone_hint: "Suporta arquivos de etiqueta (.zpl, .prn, .txt)",
            extensions: &["zpl", "prn", "txt"],
            kind: ImportKind::Templates,
        }
    }

    pub const fn datasets() -> Self {
        Self {
            title: "Importar Base de Dados",
            placeholder: "Cole os dados CSV aqui...",
            dropzone_hint: "Suporta arquivos de planilha (.csv, .txt)",
            extensions: &["csv", "txt"],
            kind: ImportKind::Datasets,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    Close,
    SelectTab(ImportTab),
    EditorAction(text_editor::Action),
    OpenFilePicker(ImportKind),
    FileSelected(Result<Option<PathBuf>, String>),
    FileHovered(bool),
    FileDropped(PathBuf),
    ImportPressed,
}

#[derive(Debug)]
pub struct State {
    pub is_open: bool,
    pub kind: Option<ImportKind>,
    pub active_tab: ImportTab,
    pub content: text_editor::Content,
    pub cached_line_numbers: String,
    pub cached_line_count: usize,
    pub selected_file: Option<PathBuf>,
    pub selected_file_size: Option<u64>,
    pub file_error: Option<String>,
    pub is_hovered: bool,
    pub is_picking_file: bool,
    pub is_importing: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_open: false,
            kind: None,
            active_tab: ImportTab::Paste,
            content: text_editor::Content::new(),
            cached_line_numbers: "1".to_string(),
            cached_line_count: 1,
            selected_file: None,
            selected_file_size: None,
            file_error: None,
            is_hovered: false,
            is_picking_file: false,
            is_importing: false,
        }
    }
}

impl State {
    pub fn reset(&mut self) {
        self.is_open = false;
        self.content = text_editor::Content::new();
        self.sync_line_numbers();
        self.selected_file = None;
        self.selected_file_size = None;
        self.file_error = None;
        self.is_hovered = false;
        self.is_picking_file = false;
        self.is_importing = false;
    }

    /// Recomputes `cached_line_numbers` when `content.line_count()` changes.
    pub fn sync_line_numbers(&mut self) {
        use std::fmt::Write as _;

        let line_count = self.content.line_count().max(1);
        if line_count != self.cached_line_count {
            self.cached_line_count = line_count;
            self.cached_line_numbers.clear();
            for i in 1..=line_count {
                if i > 1 {
                    self.cached_line_numbers.push('\n');
                }
                let _ = write!(self.cached_line_numbers, "{i}");
            }
        }
    }
}
