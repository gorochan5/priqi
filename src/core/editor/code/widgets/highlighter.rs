use std::ops::Range;

use iced::Theme;
use iced::color;
use iced::widget::text;
use iced::widget::text::highlighter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZplToken {
    Function, // Comandos ZPL (^XA, ^FO, ^A0, ^FD, ^FS, ~DG, etc.) -> #e00e68
    Bracket,  // Qualquer texto entre [...] -> #62e1e4
}

pub struct ZplParser {
    current_line: usize,
}

impl text::Parser for ZplParser {
    type Settings = ();
    type Output = ZplToken;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, ZplToken)>;

    fn new(_settings: &Self::Settings) -> Self {
        Self { current_line: 0 }
    }

    fn update(&mut self, _new_settings: &Self::Settings) {
        self.current_line = 0;
    }

    fn change_line(&mut self, line: usize) {
        self.current_line = line;
    }

    fn current_line(&self) -> usize {
        self.current_line
    }

    fn parse_line(&mut self, line: &str) -> Self::Iterator<'_> {
        self.current_line += 1;
        let mut tokens = Vec::new();
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if bytes[i] == b'^' || bytes[i] == b'~' {
                let start = i;
                i += 1;

                let cmd_start = i;
                if i < len && (bytes[i] == b'A' || bytes[i] == b'a') {
                    i += 1;
                    if i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'@') {
                        i += 1;
                    }
                } else {
                    while i < len && bytes[i].is_ascii_alphabetic() && (i - cmd_start) < 2 {
                        i += 1;
                    }
                }

                tokens.push((start..i, ZplToken::Function));
            } else if bytes[i] == b'[' {
                let start = i;
                i += 1;
                while i < len && bytes[i] != b']' {
                    i += 1;
                }
                if i < len && bytes[i] == b']' {
                    i += 1;
                    tokens.push((start..i, ZplToken::Bracket));
                }
            } else {
                i += 1;
            }
        }

        tokens.into_iter()
    }
}

pub fn zpl_highlight(token: ZplToken, _theme: &Theme) -> highlighter::Style {
    match token {
        ZplToken::Function => highlighter::Style {
            color: Some(color!(0xe00e68)),
            style: None,
        },
        ZplToken::Bracket => highlighter::Style {
            color: Some(color!(0x62e1e4)),
            style: None,
        },
    }
}

