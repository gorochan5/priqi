use iced::widget::image::Handle;
use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Code,
    Preview,
}

#[derive(Debug, Clone)]
pub struct PreviewCard {
    pub title: String,
    pub subtitle: String,
    pub image: Option<Handle>,
    pub error: Option<String>,
}

pub struct State {
    pub active_tab: Tab,
    pub content: text_editor::Content,
    pub is_focused: bool,
    pub is_dirty: bool,
    pub has_template_selected: bool,
    pub rotation: u16,
    pub label_width_mm: f32,
    pub label_height_mm: f32,
    pub label_dpi: u32,
    pub width_input: String,
    pub height_input: String,
    pub cached_line_numbers: String,
    pub cached_line_count: usize,
    pub preview_cards: Vec<PreviewCard>,
    pub maximized_card: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_tab: Tab::Code,
            content: text_editor::Content::new(),
            is_focused: false,
            is_dirty: false,
            has_template_selected: false,
            rotation: 0,
            label_width_mm: 100.0,
            label_height_mm: 150.0,
            label_dpi: 203,
            width_input: "100".to_string(),
            height_input: "150".to_string(),
            cached_line_numbers: "1".to_string(),
            cached_line_count: 1,
            preview_cards: Vec::new(),
            maximized_card: None,
        }
    }
}

impl State {
    /// Recomputes `cached_line_numbers` when `content.line_count()` changes.
    pub fn sync_line_numbers(&mut self) {
        let line_count = self.content.line_count().max(1);
        if line_count != self.cached_line_count {
            self.cached_line_count = line_count;
            self.cached_line_numbers = (1..=line_count)
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join("\n");
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTab(Tab),
    Edit(text_editor::Action),
    Unfocus,
    RotateLeft,
    RotateRight,
    LabelWidthChanged(String),
    LabelHeightChanged(String),
    LabelDpiSelected(u32),
    MaximizeCard(usize),
    UnmaximizeCard,
    PrevMaximizedCard,
    NextMaximizedCard,
}
