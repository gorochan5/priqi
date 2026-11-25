use iced::widget::{column, container, image, space, text};
use iced::{Element, Length, Widget, border, color, font};

use crate::core::icons;
use crate::core::templates::models::{Message, Template};

pub fn render_preview<'a>(template: &'a Template) -> Element<'a, Message> {
    if let Some(handle) = &template.preview_handle {
        container(
            image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fixed(185.0))
        .height(Length::Fixed(105.0))
        .padding(6)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xffffff))
                .border(border::rounded(6).color(color!(0xcbd5e1)).width(1.0))
        })
        .boxed()
    } else {
        container(
            column![
                icons::qr_code().size(22).color(color!(0x94a3b8)),
                space().height(Length::Fixed(6.0)),
                text("Sem prévia ZPL")
                    .size(10)
                    .font(font::Font {
                        weight: font::Weight::Medium,
                        ..Default::default()
                    })
                    .color(color!(0x94a3b8)),
            ]
            .align_x(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fixed(185.0))
        .height(Length::Fixed(105.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xffffff))
                .border(border::rounded(6).color(color!(0xe2e8f0)).width(1.0))
        })
        .boxed()
    }
}
