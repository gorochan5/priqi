#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrinterOption {
    OsPrinter(String),
    Pdf,
}

impl std::fmt::Display for PrinterOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OsPrinter(name) => write!(f, "{name}"),
            Self::Pdf => write!(f, "Salvar em PDF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub printer: Option<PrinterOption>,
    pub available_printers: Vec<PrinterOption>,
    pub copies: u32,
    pub collapsed: bool,
    pub print_status: Option<String>,
    pub print_status_seq: u64,
    pub can_print: bool,
    pub is_printing: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            printer: Some(PrinterOption::Pdf),
            available_printers: vec![PrinterOption::Pdf],
            copies: 1,
            collapsed: false,
            print_status: None,
            print_status_seq: 0,
            can_print: false,
            is_printing: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PrinterSelected(PrinterOption),
    CopiesChanged(u32),
    PrintPressed,
    PrintResult(Result<String, String>),
    AutoDismissPrintStatus(u64),
    ClearPrintStatus,
    ToggleCollapse,
}
