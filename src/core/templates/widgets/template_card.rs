use iced::widget::{button, column, container, mouse_area, row, space, stack, text, text_input};
use iced::{Length, Shadow, Widget, border, color, font, mouse};

use crate::core::icons;
use crate::core::templates::models::{Message, RENAME_INPUT_ID, Template};
use crate::core::templates::widgets::label_previews::render_preview;

pub fn template_card<'a>(
    template: &'a Template,
    is_selected: bool,
    is_editing: bool,
    editing_name: &'a str,
) -> impl Widget<Message> + 'a {
    // Top-right close/delete button
    let close_btn = button(
        container(icons::x().size(12))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding(3)
    .on_press(Message::DeleteTemplate(template.id.clone()))
    .style(|_theme, status| {
        let (bg, icon_color) = match status {
            button::Status::Hovered => (Some(color!(0xfee2e2).into()), color!(0xef4444)),
            button::Status::Pressed => (Some(color!(0xfecaca).into()), color!(0xdc2626)),
            _ => (Some(color!(0xffffff, 0.9).into()), color!(0x64748b)),
        };

        button::Style {
            background: bg,
            text_color: icon_color,
            border: border::rounded(14).color(color!(0xe2e8f0)).width(1.0),
            shadow: Shadow::default(),
            snap: true,
        }
    });

    let preview_box = container(
        stack![
            container(render_preview(template))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            container(
                row![
                    space().width(Length::Fill),
                    close_btn,
                ]
            )
            .width(Length::Fill)
            .padding(6),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fixed(125.0))
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xf8fafc))
            .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
    });

    let title_color = if is_selected {
        color!(0x07bc9f)
    } else {
        color!(0x1e293b)
    };

    let title_area: iced::Element<'a, Message> = if is_editing {
        row![
            text_input("Nome da etiqueta", editing_name)
                .id(RENAME_INPUT_ID)
                .on_input(Message::RenameInputChanged)
                .on_submit(Message::SaveRename)
                .size(12)
                .padding([3, 6])
                .width(Length::Fill)
                .style(|_theme, _status| {
                    text_input::Style {
                        background: color!(0xffffff).into(),
                        border: border::rounded(4).color(color!(0x07bc9f)).width(1.0),
                        placeholder: color!(0x94a3b8),
                        value: color!(0x0f172a),
                        selection: color!(0x07bc9f, 0.2),
                    }
                }),
            space().width(Length::Fixed(4.0)),
            button(
                container(icons::check().size(13).color(color!(0x07bc9f)))
                    .center_x(Length::Shrink)
                    .center_y(Length::Shrink)
            )
            .padding(4)
            .on_press(Message::SaveRename)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Some(color!(0xe8f6f3).into()),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: border::rounded(4),
                    ..button::Style::default()
                }
            }),
            space().width(Length::Fixed(2.0)),
            button(
                container(icons::x().size(13).color(color!(0xef4444)))
                    .center_x(Length::Shrink)
                    .center_y(Length::Shrink)
            )
            .padding(4)
            .on_press(Message::CancelRename)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Some(color!(0xfee2e2).into()),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: border::rounded(4),
                    ..button::Style::default()
                }
            }),
        ]
        .align_y(iced::alignment::Vertical::Center)
        .boxed()
    } else {
        let rename_btn = button(
            container(icons::pencil().size(12).color(color!(0x94a3b8)))
                .center_x(Length::Shrink)
                .center_y(Length::Shrink),
        )
        .padding(4)
        .on_press(Message::StartRename(template.id.clone()))
        .style(|_theme, status| {
            let (bg, icon_color) = match status {
                button::Status::Hovered => (Some(color!(0xe8f6f3).into()), color!(0x07bc9f)),
                button::Status::Pressed => (Some(color!(0xdcf2ee).into()), color!(0x059669)),
                _ => (None, color!(0x94a3b8)),
            };
            button::Style {
                background: bg,
                text_color: icon_color,
                border: border::rounded(4),
                ..button::Style::default()
            }
        });

        row![
            text(&template.name)
                .size(13)
                .font(font::Font {
                    weight: font::Weight::Semibold,
                    ..Default::default()
                })
                .color(title_color)
                .width(Length::Fill),
            space().width(Length::Fixed(4.0)),
            rename_btn,
        ]
        .align_y(iced::alignment::Vertical::Center)
        .boxed()
    };

    let card_content = column![
        preview_box,
        space().height(Length::Fixed(8.0)),
        title_area,
    ];

    let card_container = container(card_content)
        .width(Length::Fill)
        .padding(8)
        .style(move |_theme| {
            if is_selected {
                container::Style::default()
                    .background(color!(0xffffff))
                    .border(border::rounded(10).color(color!(0x07bc9f)).width(1.5))
            } else {
                container::Style::default()
                    .background(color!(0xffffff))
                    .border(border::rounded(10).color(color!(0xe2e8f0)).width(1.0))
            }
        });

    mouse_area(card_container)
        .interaction(mouse::Interaction::Pointer)
        .on_press(Message::SelectTemplate(template.id.clone()))
}
