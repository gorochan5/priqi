use iced::{
    Length, Widget, alignment, border, color, font, padding,
    widget::{bottom_right, button, column, container, opaque, row, space, stack, text},
};

use crate::core::icons;
use crate::core::models::{Message, State};

pub fn view(state: &State) -> impl Widget<Message> {
    app(state)
}

fn app(state: &State) -> impl Widget<Message> {
    let db_error_banner = state.db_error.as_ref().map(|err| {
        container(
            row![
                text(err).size(12).color(color!(0x991b1b)),
                space().width(Length::Fill),
                button(icons::x().size(13).color(color!(0x991b1b)))
                    .padding([2, 6])
                    .on_press(Message::DismissDbError)
                    .style(|_theme, status| button::Style {
                        background: match status {
                            button::Status::Hovered => Some(color!(0xfee2e2).into()),
                            _ => None,
                        },
                        border: border::rounded(4),
                        ..Default::default()
                    }),
            ]
            .align_y(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding([6, 14])
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xfef2f2))
                .border(border::Border {
                    color: color!(0xfecaca),
                    width: 1.0,
                    radius: 0.0.into(),
                })
        })
    });

    let main_col = column![
        crate::core::navbar::view::view(&state.navbar).map(Message::Navbar),
        db_error_banner,
        space().height(Length::Fixed(10.0)),
        row![
            space().width(Length::Fixed(10.0)),
            crate::core::datasets::view::view(&state.datasets).map(Message::Datasets),
            space().width(Length::Fixed(10.0)),
            crate::core::editor::view::view(&state.editor).map(Message::Editor),
            space().width(Length::Fixed(10.0)),
            crate::core::templates::view::view(&state.templates).map(Message::Templates),
            space().width(Length::Fixed(10.0)),
        ]
        .height(Length::Fill),
        space().height(Length::Fixed(10.0)),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    container(stack![
        main_col,
        column![
            crate::core::controls::view::view(&state.controls).map(Message::Controls),
            space().height(Length::Fixed(if state.controls.collapsed { 0.0 } else { 10.0 })),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
        state
            .controls
            .print_status
            .as_deref()
            .map(print_status_toast),
        state.templates.import_modal.is_open.then(|| {
            crate::core::widgets::import_modal::view(
                &state.templates.import_modal,
                crate::core::widgets::import_modal::ImportConfig::templates(),
            )
            .map(crate::core::templates::models::Message::ImportModal)
            .map(Message::Templates)
        }),
        state.datasets.import_modal.is_open.then(|| {
            crate::core::widgets::import_modal::view(
                &state.datasets.import_modal,
                crate::core::widgets::import_modal::ImportConfig::datasets(),
            )
            .map(crate::core::datasets::models::Message::ImportModal)
            .map(Message::Datasets)
        }),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::Style::default().background(color!(0xf4f6f5)))
}

fn print_status_toast(status: &str) -> impl Widget<Message> + '_ {
    let (bg, border_color, badge_bg, accent, icon, title, body) =
        if let Some(msg) = status.strip_prefix("✓ ") {
            (
                color!(0xf0fdf9),
                color!(0xbbf7d0),
                color!(0xd1fae5),
                color!(0x059669),
                icons::circle_check(),
                "Impressão concluída",
                msg,
            )
        } else if let Some(err) = status.strip_prefix("✗ ") {
            (
                color!(0xfef2f2),
                color!(0xfecaca),
                color!(0xfee2e2),
                color!(0xdc2626),
                icons::circle_alert(),
                "Falha na impressão",
                err,
            )
        } else {
            (
                color!(0xf8fafc),
                color!(0xe2e8f0),
                color!(0xe2e8f0),
                color!(0x475569),
                icons::printer(),
                "Imprimindo",
                status,
            )
        };

    let close_btn = button(icons::x().size(12).color(color!(0x64748b)))
        .padding([3, 5])
        .on_press(Message::Controls(
            crate::core::controls::models::Message::ClearPrintStatus,
        ))
        .style(move |_theme, btn_status| button::Style {
            background: match btn_status {
                button::Status::Hovered => Some(color!(0x000000, 0.06).into()),
                _ => None,
            },
            border: border::rounded(6),
            ..Default::default()
        });

    let badge = container(icon.size(18).color(accent))
        .width(Length::Fixed(38.0))
        .height(Length::Fixed(38.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| {
            container::Style::default()
                .background(badge_bg)
                .border(border::rounded(19))
        });

    let toast_card = container(column![
        row![space().width(Length::Fill), close_btn].width(Length::Fill),
        column![
            badge,
            text(title)
                .size(12)
                .color(color!(0x0f172a))
                .align_x(alignment::Horizontal::Center)
                .font(iced::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }),
            text(body)
                .size(11)
                .color(color!(0x475569))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .spacing(6),
    ])
    .width(Length::Fixed(185.0))
    .height(Length::Fixed(165.0))
    .padding(12)
    .style(move |_theme| {
        container::Style::default()
            .background(bg)
            .border(border::rounded(18).color(border_color).width(1.25))
    });

    bottom_right(container(opaque(toast_card)).padding(padding::right(16).bottom(16)))
}
