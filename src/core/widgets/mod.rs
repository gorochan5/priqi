use iced::widget::scrollable;
use iced::{Color, Theme, border, color};

pub mod dot_grid;
pub mod import_modal;

/// Scrollbar style for light surfaces (muted slate resting, teal hover/drag).
pub fn light_scrollbar_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    scrollbar_style(theme, status, color!(0xf1f5f9), color!(0xcbd5e1))
}

/// Scrollbar style for dark surfaces (deep olive resting, teal hover/drag).
pub fn dark_scrollbar_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    scrollbar_style(theme, status, color!(0x141a13), color!(0x374135))
}

fn scrollbar_style(
    theme: &Theme,
    status: scrollable::Status,
    rail_bg: Color,
    resting_scroller: Color,
) -> scrollable::Style {
    let base = scrollable::default(theme, status);
    let rail_background: iced::Background = rail_bg.into();

    let make_rail = |scroller_color: Color| scrollable::Rail {
        background: Some(rail_background),
        border: border::rounded(4),
        scroller: scrollable::Scroller {
            background: scroller_color.into(),
            border: border::rounded(4),
        },
    };

    let resting = make_rail(resting_scroller);
    let hovered = make_rail(color!(0x07bc9f));
    let dragged = make_rail(color!(0x06967f));

    let (vertical_rail, horizontal_rail) = match status {
        scrollable::Status::Active { .. } => (resting, resting),
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => (
            if is_vertical_scrollbar_hovered {
                hovered
            } else {
                resting
            },
            if is_horizontal_scrollbar_hovered {
                hovered
            } else {
                resting
            },
        ),
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => (
            if is_vertical_scrollbar_dragged {
                dragged
            } else {
                resting
            },
            if is_horizontal_scrollbar_dragged {
                dragged
            } else {
                resting
            },
        ),
    };

    scrollable::Style {
        vertical_rail,
        horizontal_rail,
        gap: Some(rail_background),
        ..base
    }
}
