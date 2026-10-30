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
    #[allow(dead_code)]
    pub extensions: &'static [&'static str],
    pub kind: ImportKind,
}

impl ImportConfig {
    pub const fn templates() -> Self {
        Self {
            title: "Importar Etiqueta",
            placeholder: "Cole seu código ZPL aqui...",
            dropzone_hint: "Suporta arquivos de etiqueta (.zpl, .prn)",
            extensions: &["zpl", "prn"],
            kind: ImportKind::Templates,
        }
    }

    pub const fn datasets() -> Self {
        Self {
            title: "Importar Base de Dados",
            placeholder: "Cole os dados CSV aqui...",
            dropzone_hint: "Suporta arquivos de planilha (.csv)",
            extensions: &["csv"],
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
    OpenFilePicker,
    ImportPressed,
}

#[derive(Debug)]
pub struct State {
    pub is_open: bool,
    pub active_tab: ImportTab,
    pub content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tab: ImportTab::Paste,
            content: text_editor::Content::new(),
        }
    }
}
