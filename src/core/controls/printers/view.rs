use iced::Length;
use iced::Widget;
use iced::widget::column;
use iced::widget::space;
use iced::widget::text;
use iced::widget::{container, pick_list, row};

use crate::core::controls::printers::models::{Message, Printer};
use crate::core::icons;

pub fn view(_state: &()) -> impl Widget<Message> {
    controls(_state)
}

fn controls(_state: &()) -> impl Widget<Message> {
    let select = pick_list(
        Some(Printer::Thermal),
        &Printer::ALL[..],
        Printer::to_string,
    )
    .placeholder("Selecione a impressora...")
    .on_select(Message::PrinterSelected)
    .width(Length::Fixed(180.0));

    container(
        column![
            select,
            space().height(Length::Fill),
            row![icons::printer().size(12), space().width(Length::Fixed(5.0)), text("Impressoras").size(8)],
        ]
    )
    .padding(5)
}
