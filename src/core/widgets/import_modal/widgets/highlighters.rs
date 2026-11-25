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
                b',' | b';' | b'\t' => {
                    tokens.push((i..i + 1, CsvToken::Delimiter));
                    i += 1;
                }
                b'"' => {
                    let start = i;
                    i += 1;
                    while i < len {
                        if bytes[i] == b'"' {
                            if i + 1 < len && bytes[i + 1] == b'"' {
                                i += 2;
                            } else {
                                i += 1;
                                break;
                            }
                        } else {
                            i += 1;
                        }
                    }
                    tokens.push((start..i, CsvToken::Quoted));
                }
                _ => {
                    let start = i;
                    while i < len
                        && bytes[i] != b','
                        && bytes[i] != b';'
                        && bytes[i] != b'\t'
                        && bytes[i] != b'"'
                    {
                        i += 1;
                    }
                    let token_kind = if is_csv_number(&line[start..i]) {
                        CsvToken::Number
                    } else {
                        CsvToken::Field
                    };
                    tokens.push((start..i, token_kind));
                }
            }
        }

        tokens.into_iter()
    }
}

fn is_csv_number(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    let s = s.strip_prefix(['-', '+']).unwrap_or(s);
    if s.is_empty() {
        return false;
    }
    let mut seen_dot = false;
    let mut seen_digit = false;
    for b in s.bytes() {
        if b.is_ascii_digit() {
            seen_digit = true;
        } else if b == b'.' && !seen_dot {
            seen_dot = true;
        } else {
            return false;
        }
    }
    seen_digit
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

