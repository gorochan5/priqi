use iced::Element;
use iced::Length;
use iced::Radians;
use iced::Rotation;
use iced::Shadow;
use iced::Vector;
use iced::Widget;
use iced::alignment;
use iced::border;
use iced::color;
use iced::font;
use iced::mouse;
use iced::overlay::menu;
use iced::widget::{
    button, column, container, image, mouse_area, pick_list, row, rule, scrollable, space, text,
    text_input,
};

use crate::core::editor::models::{Message, PreviewCard, State};
use crate::core::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DpiOption(u32);

impl DpiOption {
    const ALL: [Self; 3] = [Self(203), Self(300), Self(600)];
}

impl std::fmt::Display for DpiOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} DPI", self.0)
    }
}

pub fn preview_frame(state: &State) -> Element<'_, Message> {
    let total_cards = state.preview_cards.len();
    let is_multi = total_cards > 1;
    let maximized_idx = state
        .maximized_card
        .filter(|&idx| idx < total_cards);

    // Barra superior de controles da visualização (status, navegação e botões de girar etiqueta)
    let left_controls: Element<'_, Message> = if let Some(idx) = maximized_idx {
        let back_btn = button(
            row![
                icons::arrow_left().size(14).color(color!(0x07bc9f)),
                space().width(Length::Fixed(6.0)),
                text("Voltar aos cards")
                    .size(12)
                    .font(font::Font {
                        weight: font::Weight::Semibold,
                        ..Default::default()
                    })
                    .color(color!(0x07bc9f)),
            ]
            .align_y(alignment::Vertical::Center),
        )
        .padding([5, 10])
        .on_press(Message::UnmaximizeCard)
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered => color!(0xdcf2ee),
                _ => color!(0xe8f6f3),
            };
            button::Style {
                background: Some(bg.into()),
                text_color: color!(0x07bc9f),
                border: border::rounded(6).color(color!(0xb6e3db)).width(1.0),
                ..Default::default()
            }
        });

        let prev_btn = icon_action_button(
            icons::chevron_left().size(15),
            (idx > 0).then_some(Message::PrevMaximizedCard),
        );
        let next_btn = icon_action_button(
            icons::chevron_right().size(15),
            (idx + 1 < total_cards).then_some(Message::NextMaximizedCard),
        );

        row![
            back_btn,
            space().width(Length::Fixed(10.0)),
            prev_btn,
            space().width(Length::Fixed(6.0)),
            text(format!("Etiqueta {} de {}", idx + 1, total_cards))
                .size(12)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x334155)),
            space().width(Length::Fixed(6.0)),
            next_btn,
        ]
        .align_y(alignment::Vertical::Center)
        .boxed()
    } else if is_multi {
        row![
            container(
                text(format!("{total_cards} etiquetas"))
                    .size(11)
                    .font(font::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(color!(0x065f46)),
            )
            .padding([3, 8])
            .style(|_theme| {
                container::Style::default()
                    .background(color!(0xd1fae5))
                    .border(border::rounded(10).color(color!(0xa7f3d0)).width(1.0))
            }),
            space().width(Length::Fixed(8.0)),
            text("Clique em qualquer card abaixo para maximizar a visualização")
                .size(12)
                .color(color!(0x64748b)),
        ]
        .align_y(alignment::Vertical::Center)
        .boxed()
    } else if let Some(first) = state.preview_cards.first() {
        row![
            text(&first.title)
                .size(12)
                .font(font::Font {
                    weight: font::Weight::Semibold,
                    ..Default::default()
                })
                .color(color!(0x334155)),
            space().width(Length::Fixed(8.0)),
            text(&first.subtitle).size(12).color(color!(0x64748b)),
        ]
        .align_y(alignment::Vertical::Center)
        .boxed()
    } else {
        text("Pré-visualização ZPL")
            .size(12)
            .color(color!(0x64748b))
            .boxed()
    };

    let rotate_left_btn = icon_action_button(
        icons::rotate_ccw().size(15),
        Some(Message::RotateLeft),
    );
    let rotate_right_btn = icon_action_button(
        icons::rotate_cw().size(15),
        Some(Message::RotateRight),
    );

    let width_input = text_input("100", &state.width_input)
        .on_input(Message::LabelWidthChanged)
        .width(Length::Fixed(46.0))
        .size(11)
        .padding([4, 6])
        .style(|theme, status| {
            let border_color = match status {
                text_input::Status::Focused { .. } => color!(0x07bc9f),
                text_input::Status::Hovered => color!(0x94a3b8),
                _ => color!(0xe2e8f0),
            };
            let mut style = text_input::default(theme, status);
            style.background = color!(0xf8fafc).into();
            style.border = border::rounded(6).color(border_color).width(1.0);
            style.placeholder = color!(0x94a3b8);
            style.value = color!(0x1e293b);
            style.selection = color!(0xb6e3db);
            style
        });

    let height_input = text_input("150", &state.height_input)
        .on_input(Message::LabelHeightChanged)
        .width(Length::Fixed(46.0))
        .size(11)
        .padding([4, 6])
        .style(|theme, status| {
            let border_color = match status {
                text_input::Status::Focused { .. } => color!(0x07bc9f),
                text_input::Status::Hovered => color!(0x94a3b8),
                _ => color!(0xe2e8f0),
            };
            let mut style = text_input::default(theme, status);
            style.background = color!(0xf8fafc).into();
            style.border = border::rounded(6).color(border_color).width(1.0);
            style.placeholder = color!(0x94a3b8);
            style.value = color!(0x1e293b);
            style.selection = color!(0xb6e3db);
            style
        });

    let dpi_picker = pick_list(
        Some(DpiOption(state.label_dpi)),
        &DpiOption::ALL[..],
        DpiOption::to_string,
    )
    .on_select(|opt| Message::LabelDpiSelected(opt.0))
    .width(Length::Fixed(86.0))
    .text_size(11)
    .padding([4, 8])
    .style(|theme, status| {
        let mut style = pick_list::default(theme, status);
        style.background = color!(0xf8fafc).into();
        let border_color = match status {
            pick_list::Status::Hovered | pick_list::Status::Opened { .. } => color!(0x07bc9f),
            _ => color!(0xe2e8f0),
        };
        style.border = border::rounded(6).color(border_color).width(1.0);
        style.text_color = color!(0x334155);
        style.handle_color = color!(0x07bc9f);
        style
    })
    .menu_style(|theme| {
        let mut style = menu::default(theme);
        style.background = color!(0xffffff).into();
        style.border = border::rounded(6).color(color!(0xe2e8f0)).width(1.0);
        style.text_color = color!(0x1e293b);
        style.selected_background = color!(0x07bc9f).into();
        style.selected_text_color = color!(0xffffff);
        style
    });

    let mut right_controls = row![
        text("L").size(11).color(color!(0x64748b)),
        space().width(Length::Fixed(4.0)),
        width_input,
        space().width(Length::Fixed(4.0)),
        text("×").size(11).color(color!(0x64748b)),
        space().width(Length::Fixed(4.0)),
        text("A").size(11).color(color!(0x64748b)),
        space().width(Length::Fixed(4.0)),
        height_input,
        space().width(Length::Fixed(4.0)),
        text("mm").size(11).color(color!(0x64748b)),
        space().width(Length::Fixed(8.0)),
        dpi_picker,
        space().width(Length::Fixed(10.0)),
        rotate_left_btn,
        space().width(Length::Fixed(4.0)),
        container(
            text(format!("{}°", state.rotation))
                .size(11)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x475569)),
        )
        .padding([4, 8])
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xf1f5f9))
                .border(border::rounded(6).color(color!(0xe2e8f0)).width(1.0))
        }),
        space().width(Length::Fixed(4.0)),
        rotate_right_btn,
    ]
    .align_y(alignment::Vertical::Center);

    if maximized_idx.is_some() {
        right_controls = right_controls
            .push(space().width(Length::Fixed(8.0)).boxed())
            .push(icon_action_button(
                icons::minimize_2().size(15),
                Some(Message::UnmaximizeCard),
            ));
    }

    let toolbar = container(
        row![
            left_controls,
            space().width(Length::Fill),
            right_controls,
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding([8, 12])
    .width(Length::Fill)
    .style(|_theme| container::Style::default().background(color!(0xffffff)));

    // Corpo principal
    let body: Element<'_, Message> = if let Some(idx) = maximized_idx {
        render_single_card_maximized(state, &state.preview_cards[idx], is_multi)
    } else if is_multi {
        render_cards_grid(state, &state.preview_cards)
    } else if let Some(card) = state.preview_cards.first() {
        render_single_card_maximized(state, card, false)
    } else {
        let empty_msg = if !state.has_template_selected {
            "Nenhum template ZPL selecionado para pré-visualização."
        } else {
            "Digite comandos ZPL válidos para pré-visualizar a etiqueta."
        };
        container(
            text(empty_msg)
                .size(14)
                .color(color!(0x64748b)),
        )
        .center(Length::Fill)
        .boxed()
    };

    container(
        column![
            toolbar,
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xe2e8f0),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            container(body)
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xf1f5f9))
            .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
    })
    .boxed()
}

fn render_cards_grid<'a>(state: &State, cards: &'a [PreviewCard]) -> Element<'a, Message> {
    let mut grid_col = column![].spacing(14).padding(16).width(Length::Fill);

    let mut idx = 0;
    while idx < cards.len() {
        let first_card = container(render_card_item(state, idx, &cards[idx]))
            .width(Length::FillPortion(1))
            .height(Length::Shrink);
        let row_el: Element<'a, Message> = if idx + 1 < cards.len() {
            let second_card = container(render_card_item(state, idx + 1, &cards[idx + 1]))
                .width(Length::FillPortion(1))
                .height(Length::Shrink);
            row![first_card, second_card]
                .spacing(14)
                .width(Length::Fill)
                .boxed()
        } else {
            row![
                first_card,
                space().width(Length::FillPortion(1)),
            ]
            .spacing(14)
            .width(Length::Fill)
            .boxed()
        };

        grid_col = grid_col.push(row_el);
        idx += 2;
    }

    scrollable(grid_col)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::default(),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(crate::core::widgets::light_scrollbar_style)
        .boxed()
}

fn render_card_item<'a>(
    state: &State,
    card_idx: usize,
    card: &'a PreviewCard,
) -> Element<'a, Message> {
    let header = row![
        container(
            text(format!("#{}", card_idx + 1))
                .size(11)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x07bc9f)),
        )
        .padding([2, 6])
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xe8f6f3))
                .border(border::rounded(6).color(color!(0xb6e3db)).width(1.0))
        }),
        space().width(Length::Fixed(8.0)),
        column![
            text(&card.title)
                .size(13)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x1e293b))
                .wrapping(text::Wrapping::None)
                .ellipsis(text::Ellipsis::End),
            text(&card.subtitle)
                .size(11)
                .color(color!(0x64748b))
                .wrapping(text::Wrapping::None)
                .ellipsis(text::Ellipsis::End),
        ]
        .width(Length::Fill),
        container(icons::maximize_2().size(14).color(color!(0x07bc9f)))
            .padding(4)
            .style(|_theme| {
                container::Style::default()
                    .background(color!(0xf8fafc))
                    .border(border::rounded(6).color(color!(0xe2e8f0)).width(1.0))
            }),
    ]
    .width(Length::Fill)
    .align_y(alignment::Vertical::Center);

    let rotation = Rotation::Solid(Radians((state.rotation as f32).to_radians()));

    let preview_area: Element<'a, Message> = if let Some(handle) = &card.image {
        container(
            image(handle.clone())
                .content_fit(iced::ContentFit::Contain)
                .rotation(rotation)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(185.0))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xf8fafc))
                .border(border::rounded(6).color(color!(0xe2e8f0)).width(1.0))
        })
        .boxed()
    } else if let Some(err) = &card.error {
        container(
            text(err)
                .size(11)
                .color(color!(0xef4444)),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(185.0))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xfef2f2))
                .border(border::rounded(6).color(color!(0xfecaca)).width(1.0))
        })
        .boxed()
    } else {
        container(space())
            .width(Length::Fill)
            .height(Length::Fixed(185.0))
            .boxed()
    };

    let card_box = container(
        column![
            header,
            space().height(Length::Fixed(10.0)),
            preview_area,
        ]
        .width(Length::Fill),
    )
    .padding(12)
    .width(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xffffff))
            .border(border::rounded(10).color(color!(0xcbd5e1)).width(1.0))
            .shadow(Shadow {
                offset: Vector::new(0.0, 2.0),
                blur_radius: 6.0,
                color: color!(0x000000, 0.06),
            })
    });

    mouse_area(card_box)
        .interaction(mouse::Interaction::Pointer)
        .on_press(Message::MaximizeCard(card_idx))
        .boxed()
}

fn render_single_card_maximized<'a>(
    state: &State,
    card: &'a PreviewCard,
    show_info_pill: bool,
) -> Element<'a, Message> {
    let rotation = Rotation::Solid(Radians((state.rotation as f32).to_radians()));

    if let Some(handle) = &card.image {
        let label_card = container(
            image(handle.clone())
                .rotation(rotation)
                .width(Length::Shrink)
                .height(Length::Shrink),
        )
        .padding(16)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xffffff))
                .border(border::rounded(8).color(color!(0xcbd5e1)).width(1.0))
                .shadow(Shadow {
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                    color: color!(0x000000, 0.12),
                })
        });

        let mut content_col = column![space().height(Length::Fixed(16.0))]
            .align_x(alignment::Horizontal::Center)
            .width(Length::Fill);

        if show_info_pill && !card.subtitle.is_empty() {
            content_col = content_col.push(
                container(
                    row![
                        text(&card.title)
                            .size(12)
                            .font(font::Font {
                                weight: font::Weight::Bold,
                                ..Default::default()
                            })
                            .color(color!(0x0f172a))
                            .wrapping(text::Wrapping::None)
                            .ellipsis(text::Ellipsis::End),
                        space().width(Length::Fixed(8.0)),
                        text(&card.subtitle)
                            .size(12)
                            .color(color!(0x475569))
                            .wrapping(text::Wrapping::None)
                            .ellipsis(text::Ellipsis::End),
                    ]
                    .align_y(alignment::Vertical::Center),
                )
                .padding([6, 12])
                .style(|_theme| {
                    container::Style::default()
                        .background(color!(0xffffff))
                        .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
                })
                .boxed(),
            );
            content_col = content_col.push(space().height(Length::Fixed(12.0)).boxed());
        }

        content_col = content_col
            .push(label_card.boxed())
            .push(space().height(Length::Fixed(20.0)).boxed());

        container(
            scrollable(content_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(crate::core::widgets::light_scrollbar_style),
        )
        .center(Length::Fill)
        .boxed()
    } else if let Some(err) = &card.error {
        container(
            column![
                text("Não foi possível renderizar a etiqueta")
                    .size(15)
                    .color(color!(0xef4444)),
                space().height(Length::Fixed(6.0)),
                text(err).size(12).color(color!(0x64748b)),
            ]
            .align_x(alignment::Horizontal::Center),
        )
        .center(Length::Fill)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xfef2f2))
                .border(border::rounded(8).color(color!(0xfecaca)).width(1.0))
        })
        .boxed()
    } else {
        container(
            text("Digite comandos ZPL válidos para pré-visualizar a etiqueta.")
                .size(14)
                .color(color!(0x64748b)),
        )
        .center(Length::Fill)
        .boxed()
    }
}

fn icon_action_button<'a>(
    icon: iced::widget::Text<'a>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let enabled = on_press.is_some();
    let icon_color = if enabled {
        color!(0x334155)
    } else {
        color!(0xcbd5e1)
    };

    button(
        container(icon.color(icon_color))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding([5, 8])
    .on_press_maybe(on_press)
    .style(move |_theme, status| {
        let bg = match status {
            button::Status::Hovered if enabled => color!(0xe8f6f3),
            button::Status::Pressed if enabled => color!(0xdcf2ee),
            _ => color!(0xf8fafc),
        };
        let border_col = match status {
            button::Status::Hovered if enabled => color!(0x07bc9f),
            _ => color!(0xe2e8f0),
        };
        button::Style {
            background: Some(bg.into()),
            text_color: icon_color,
            border: border::rounded(6).color(border_col).width(1.0),
            ..Default::default()
        }
    })
    .boxed()
}
