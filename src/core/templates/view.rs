use iced::widget::{button, column, container, row, rule, scrollable, space, text};
use iced::{Length, Shadow, Vector, Widget, alignment, border, color};

use crate::core::icons::{copy, plus, qr_code};
use crate::core::templates::models::{Message, State};
use crate::core::templates::widgets::template_card::template_card;
use crate::core::widgets::import_modal::upload_button;

pub fn view<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    templates(state)
}

fn templates<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    let mut cards = column![].spacing(12).padding(2);
    for template in &state.templates {
        let is_selected = state.selected_id.as_deref() == Some(&template.id);
        let is_editing = state.editing_id.as_deref() == Some(&template.id);
        cards = cards.push(template_card(
            template,
            is_selected,
            is_editing,
            &state.editing_name,
        ));
    }

    container(
        column![
            row![
                qr_code().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                column![
                    text("").size(5),
                    text("TEMPLATES").size(14).color(color!(0x64748b)),
                ],
                space().width(Length::Fill),
                duplicate_button(),
                space().width(Length::Fixed(4.0)),
                add_button(),
                space().width(Length::Fixed(4.0)),
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
            scrollable(cards)
                .height(Length::Fill)
                .width(Length::Fill),
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

fn duplicate_button<'a>() -> impl Widget<Message> + 'a {
    button(
        container(copy().size(16).color(color!(0x07bc9f)))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding(4)
    .on_press(Message::DuplicateTemplate)
    .style(|_theme, status| {
        let background = match status {
            button::Status::Hovered => Some(color!(0xe8f6f3).into()),
            button::Status::Pressed => Some(color!(0xdcf2ee).into()),
            _ => None,
        };

        button::Style {
            background,
            text_color: color!(0x07bc9f),
            border: border::rounded(6),
            shadow: Shadow::default(),
            snap: true,
        }
    })
}

fn add_button<'a>() -> impl Widget<Message> + 'a {
    button(
        container(plus().size(18).color(color!(0x07bc9f)))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding(4)
    .on_press(Message::AddTemplate)
    .style(|_theme, status| {
        let background = match status {
            button::Status::Hovered => Some(color!(0xe8f6f3).into()),
            button::Status::Pressed => Some(color!(0xdcf2ee).into()),
            _ => None,
        };

        button::Style {
            background,
            text_color: color!(0x07bc9f),
            border: border::rounded(6),
            shadow: Shadow::default(),
            snap: true,
        }
    })
}
