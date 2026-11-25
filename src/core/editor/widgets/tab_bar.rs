use iced::Background;
use iced::Font;
use iced::Length;
use iced::Shadow;
use iced::Widget;
use iced::alignment;
use iced::border;
use iced::color;
use iced::font;
use iced::widget::{button, container, row, space, text};

use crate::core::editor::models::{Message, State, Tab};
use crate::core::icons;

pub fn tab_bar(state: &State) -> impl Widget<Message> {
    container(
        row![
            tab_button(
                icons::code(),
                "Código",
                Tab::Code,
                state.active_tab == Tab::Code,
            ),
            space().width(Length::Fixed(8.0)),
            tab_button(
                icons::eye(),
                "Visualização",
                Tab::Preview,
                state.active_tab == Tab::Preview,
            ),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding([8, 12])
}

fn tab_button<'a>(
    icon: text::Text<'a>,
    label: &'a str,
    tab: Tab,
    is_active: bool,
) -> impl Widget<Message> + 'a {
    let text_color = if is_active {
        color!(0x0d7363)
    } else {
        color!(0x526071)
    };

    let icon_color = if is_active {
        color!(0x12947f)
    } else {
        color!(0x64748b)
    };

    let font_weight = if is_active {
        font::Weight::Bold
    } else {
        font::Weight::Medium
    };

    button(
        row![
            icon.size(16).color(icon_color),
            space().width(Length::Fixed(8.0)),
            text(label)
                .size(14)
                .color(text_color)
                .font(Font {
                    weight: font_weight,
                    ..Default::default()
                }),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding([8, 14])
    .on_press_maybe((!is_active).then_some(Message::SelectTab(tab)))
    .style(move |_theme, status| {
        if is_active {
            let bg = match status {
                button::Status::Hovered => color!(0xdcf2ee),
                button::Status::Pressed => color!(0xccebe5),
                _ => color!(0xe8f6f3),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color,
                border: border::rounded(8).color(color!(0x8fd6c8)).width(1.0),
                shadow: Shadow::default(),
                snap: true,
            }
        } else {
            let bg = match status {
                button::Status::Hovered => Some(Background::Color(color!(0xf1f5f9))),
                button::Status::Pressed => Some(Background::Color(color!(0xe2e8f0))),
                _ => None,
            };
            button::Style {
                background: bg,
                text_color,
                border: border::rounded(8),
                shadow: Shadow::default(),
                snap: true,
            }
        }
    })
}

