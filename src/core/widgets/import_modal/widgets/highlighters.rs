use std::ops::Range;

use iced::widget::text::{self, highlighter};
use iced::{Theme, color};

pub use crate::core::editor::code::widgets::highlighter::{ZplParser, zpl_highlight};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CsvToken {
    Delimiter,
    Quoted,
    Number,
    Field,
}

pub struct CsvParser {
    current_line: usize,
}

impl text::Parser for CsvParser {
    type Settings = ();
    type Output = CsvToken;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, CsvToken)>;

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
            match bytes[i] {
                b',' | b';' => {
                    tokens.push((i..i + 1, CsvToken::Delimiter));
                    i += 1;
                }
                b'"' => {
                    let start = i;
                    i += 1;
                    while i < len && bytes[i] != b'"' {
                        i += 1;
                    }
                    if i < len && bytes[i] == b'"' {
                        i += 1;
                    }
                    tokens.push((start..i, CsvToken::Quoted));
                }
                b'0'..=b'9' | b'-' => {
                    let start = i;
                    i += 1;
                    while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                        i += 1;
                    }
                    tokens.push((start..i, CsvToken::Number));
                }
                _ => {
                    let start = i;
                    while i < len && bytes[i] != b',' && bytes[i] != b';' && bytes[i] != b'"' {
                        i += 1;
                    }
                    tokens.push((start..i, CsvToken::Field));
                }
            }
        }

        tokens.into_iter()
    }
}

pub fn csv_highlight(token: CsvToken, _theme: &Theme) -> highlighter::Style {
    match token {
        CsvToken::Delimiter => highlighter::Style {
            color: Some(color!(0x07bc9f)),
            style: None,
        },
        CsvToken::Quoted => highlighter::Style {
            color: Some(color!(0x62e1e4)),
            style: None,
        },
        CsvToken::Number => highlighter::Style {
            color: Some(color!(0xf59e0b)),
            style: None,
        },
        CsvToken::Field => highlighter::Style {
            color: Some(color!(0xfffff8)),
            style: None,
        },
    }
}

