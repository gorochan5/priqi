use crate::core::{icons, navbar::models::Message};
use iced::widget::text::Alignment;
use iced::{
    Length, Widget,
    border::{self, rounded},
    color,
    widget::{Button, Image, button, column, container, row, space, text},
};

pub fn render<'a>(_state: &()) -> impl Widget<Message> + 'a {
    container(text("v0.1.0").size(9).center())
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(16.0))
        .padding(5)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xe6f4f1)) // Fundo verde/menta bem claro
                .color(color!(0x0d8276)) // Cor do texto verde-azulado escuro
                .border(
                    border::rounded(10) // Raio do arredondamento (pílula)
                        .color(color!(0xb6e3db)) // Borda sutil
                        .width(1.0),
                )
        })
}
