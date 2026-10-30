use iced::widget::{column, container, row, rule, space, text};
use iced::{Length, Shadow, Vector, Widget, alignment, border, color};

use crate::core::datasets::models::{Message, State};
use crate::core::datasets::widgets::dataset_table::dataset_table;
use crate::core::datasets::widgets::datasets_dropdown::datasets_dropdown;
use crate::core::icons::database;
use crate::core::widgets::import_modal::upload_button;

pub fn view<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    datasets(state)
}

fn datasets<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    container(
        column![
            row![
                database().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                column![
                    text("").size(5),
                    text("DATASETS").size(14).color(color!(0x64748b)),
                ],
                space().width(Length::Fixed(10.0)),
                column![
                    datasets_dropdown(state),
                    space().height(Length::Fixed(4.0)),
                ],
                space().width(Length::Fill),
                upload_button(&state.import_modal).map(Message::ImportModal),
            ]
            .align_y(alignment::Vertical::Center),
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xeaeaea),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            space().height(Length::Fixed(6.0)),
            dataset_table(state),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
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
