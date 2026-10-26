use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Code,
    Preview,
}

pub struct State {
    pub active_tab: Tab,
    pub content: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        let default_zpl = "^XA\n^FO50,50^A0N,30,30^FD[NOME_PRODUTO]^FS\n^FO50,100^BY2^BCN,60,Y,N,N^FD[CODIGO_BARRAS]^FS\n^XZ";
        Self {
            active_tab: Tab::Code,
            content: text_editor::Content::with_text(default_zpl),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTab(Tab),
    Edit(text_editor::Action),
}

