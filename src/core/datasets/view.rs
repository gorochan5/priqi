use iced::widget::{column, container, rule, row};
use iced::widget::text;
use iced::widget::space;
use iced::{Length, Shadow, Vector, Widget, border, color};

use crate::core::datasets::models::{Message, State};
use crate::core::icons::database;

pub fn view(state: &State) -> impl Widget<Message> {
    datasets(state)
}

fn datasets(state: &State) -> impl Widget<Message> {
    container(
        column![
            row![
                database().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                column![
                    text("").size(5),
                    text("DATASETS").size(14).color(color!(0x64748b)),
                ]
            ],
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xeaeaea),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            // things here i guess
            text("Datasets")
            ]
            .width(Length::Fill)
            .height(Length::Fill)
        )
        .width(Length::FillPortion(20))
        .height(Length::Fill)
        .padding(5)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xfefefe))
                .border(border::rounded(10).color(color!(0xeaeaea)).width(1.5))
                .shadow(Shadow {
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 5.0,
                    color: color!(0x000000, 0.1),
                })
        })
}
