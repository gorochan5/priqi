use crate::core::navbar::models::{Message, State};
use iced::{
    Length, Widget, border, color,
    widget::{container, text},
};

pub fn render<'a>(_state: &State) -> impl Widget<Message> + 'a {
    container(text(concat!("v", env!("CARGO_PKG_VERSION"))).size(9).center())
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding([2, 8])
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
