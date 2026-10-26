#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Printer {
    Thermal,
    Laser,
    Pdf,
}

impl Printer {
    pub const ALL: [Printer; 3] = [Printer::Thermal, Printer::Laser, Printer::Pdf];
}

impl std::fmt::Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thermal => write!(f, "Térmica 80mm"),
            Self::Laser => write!(f, "Laser Escritório"),
            Self::Pdf => write!(f, "Salvar em PDF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub printer: Option<Printer>,
    pub copies: u32,
    pub collapsed: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            printer: Some(Printer::Thermal),
            copies: 39,
            collapsed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PrinterSelected(Printer),
    CopiesChanged(u32),
    PrintPressed,
    ToggleCollapse,
    Nothing,
}
