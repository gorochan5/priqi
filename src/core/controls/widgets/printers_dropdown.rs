use iced::overlay::menu;
use iced::widget::column;
use iced::widget::space;
use iced::widget::text;
use iced::widget::{pick_list, row};
use iced::{Length, Shadow, Vector, Widget, border, color};

use crate::core::controls::models::{Message, Printer, State};
use crate::core::icons;

pub fn printers_dropdown(state: &State) -> impl Widget<Message> {
    // IMPRESSORA
    let printers_dropdown = pick_list(state.printer, &Printer::ALL[..], Printer::to_string)
        .placeholder("Selecione a impressora...")
        .on_select(Message::PrinterSelected)
        .width(Length::Fixed(180.0))
        .style(|theme, status| {
            let mut style = pick_list::default(theme, status);
            style.background = color!(0xffffff).into();
            style.border = border::rounded(8).color(color!(0xe2e8f0)).width(1.0);
            style.text_color = color!(0x1e293b);
            style.handle_color = color!(0x07bc9f);
            style
        })
        .menu_style(|theme| {
            let mut style = menu::default(theme);
            style.background = color!(0xffffff).into();
            style.border = border::rounded(0).color(color!(0xe2e8f0)).width(1.0);
            style.text_color = color!(0x1e293b);
            style.selected_background = color!(0x07bc9f).into();
            style.selected_text_color = color!(0xffffff);
            style.shadow = Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
                color: color!(0x000000, 0.08),
            };
            style
        });

    column![
        printers_dropdown,
        space().height(Length::Fixed(3.0)),
        row![
            icons::printer().size(24).color(color!(0x07bc9f)),
            space().width(Length::Fixed(5.0)),
            column![
                text("").size(5),
                text("IMPRESSORA").size(14).color(color!(0x64748b))
            ]
        ],
    ]
}
