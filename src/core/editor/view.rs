use iced::widget::{column, container, rule, row};
use iced::widget::text;
use iced::widget::space;
use iced::{Length, Shadow, Vector, Widget, alignment, border, color};

use crate::core::editor::code;
use crate::core::editor::models::{Message, State, Tab};
use crate::core::editor::preview;
use crate::core::editor::widgets::tab_bar::tab_bar;
use crate::core::icons::code;

pub fn view(state: &State) -> impl Widget<Message> {
    editor(state)
}

fn editor(state: &State) -> impl Widget<Message> {
    let header = tab_bar(state);

    let body = match state.active_tab {
        Tab::Code => code::view::view(state).boxed(),
        Tab::Preview => preview::view::view(state).boxed(),
    };

    container(
        column![
            row![
                code().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                text("EDITOR").size(14).color(color!(0x64748b)),
            ]
            .align_y(alignment::Vertical::Center),
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xeaeaea),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            header, 
            body
            ]
            .width(Length::Fill)
            .height(Length::Fill)
        )
        .width(Length::FillPortion(60))
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
