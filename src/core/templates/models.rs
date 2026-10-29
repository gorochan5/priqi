use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UploadTab {
    #[default]
    Paste,
    File,
}

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    OpenUploadModal,
    CloseUploadModal,
    SelectUploadTab(UploadTab),
    CodeEditorAction(text_editor::Action),
    OpenFilePicker,
    ImportPressed,
}

pub struct State {
    pub nothing: String,
    pub show_upload_modal: bool,
    pub upload_tab: UploadTab,
    pub paste_content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        Self {
            nothing: String::new(),
            show_upload_modal: false,
            upload_tab: UploadTab::Paste,
            paste_content: text_editor::Content::new(),
        }
    }
}
