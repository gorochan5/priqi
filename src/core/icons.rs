//! Lucide icons rendered from the embedded `lucide.ttf`.
//!
//! Codepoints come from `resources/lucide/lucide.css` (same version as the `.ttf`).

use iced::Font;
use iced::widget::image::{Handle, Image};
use iced::widget::{Text, image, text};

/// Raw bytes of the Lucide font, embedded in the binary.
pub const FONT_BYTES: &[u8] = include_bytes!("../../resources/lucide/lucide.ttf");

/// Font family declared inside `lucide.ttf`.
pub const FONT: Font = Font::new("lucide");

fn icon<'a>(codepoint: char) -> Text<'a> {
    text(codepoint).font(FONT)
}

pub fn minus<'a>() -> Text<'a> {
    icon('\u{e11c}')
}

pub fn maximize<'a>() -> Text<'a> {
    icon('\u{e112}')
}

pub fn x<'a>() -> Text<'a> {
    icon('\u{e1b2}')
}

pub fn printer<'a>() -> Text<'a> {
    icon('\u{e141}')
}

pub fn branding() -> Image {
    image(Handle::from_bytes(&include_bytes!("../../resources/branding.png")[..]))
}