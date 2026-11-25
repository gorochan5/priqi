use iced::widget::{
    button, column, container, mouse_area, row, rule, scrollable, space, text, text_input,
};
use iced::{Element, Length, Widget, alignment, border, color, font, mouse};

use crate::core::datasets::models::{
    CELL_INPUT_ID, CellCoord, Message, State, TABLE_ROWS_SCROLLABLE_ID, TABLE_SCROLLABLE_ID,
};
use crate::core::datasets::update::filtered_row_indices;
use crate::core::icons;

pub fn dataset_table<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    let search_bar = container(
        row![
            icons::search().size(15).color(color!(0x64748b)),
            space().width(Length::Fixed(8.0)),
            text_input(state.search_placeholder(), &state.search_query)
                .on_input(Message::SearchChanged)
                .size(13)
                .width(Length::Fill)
                .style(|_theme, _status| text_input::Style {
                    background: color!(0x000000, 0.0).into(),
                    border: border::rounded(0),
                    placeholder: color!(0x94a3b8),
                    value: color!(0x1e293b),
                    selection: color!(0x07bc9f, 0.2),
                }),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .padding([8, 12])
    .style(|_theme| container::Style::default().background(color!(0xf8fafc)));

    let table_content: Element<'a, Message> = if let Some(table) = state.current_table() {
        let col_count = table.headers.len();
        const COL_WIDTH: f32 = 140.0;
        const SELECT_COL_WIDTH: f32 = 34.0;
        let total_width = SELECT_COL_WIDTH
            + 1.0
            + (col_count as f32) * COL_WIDTH
            + (col_count.saturating_sub(1) as f32) * 1.0;

        // 1. Headers Row (strictly fixed to 36px)
        let matching_indices = filtered_row_indices(table, &state.search_query);
        let all_selected = !matching_indices.is_empty()
            && matching_indices
                .iter()
                .all(|idx| state.selected_rows.contains(idx));
        let some_selected = !matching_indices.is_empty()
            && matching_indices
                .iter()
                .any(|idx| state.selected_rows.contains(idx));
        let select_all_icon = if all_selected {
            icons::check_square().size(15).color(color!(0x07bc9f))
        } else if some_selected {
            icons::check_square_2().size(15).color(color!(0x07bc9f))
        } else {
            icons::square().size(15).color(color!(0x94a3b8))
        };

        let select_all_btn = button(
            container(select_all_icon)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fixed(SELECT_COL_WIDTH))
        .height(Length::Fixed(36.0))
        .padding(0)
        .on_press(Message::SelectAllRows)
        .style(|_theme, _status| button::Style {
            background: None,
            border: border::rounded(0),
            ..Default::default()
        });

        let mut header_row = row![
            select_all_btn.boxed(),
            container(space())
                .width(Length::Fixed(1.0))
                .height(Length::Fixed(36.0))
                .style(|_theme| container::Style::default().background(color!(0xcbd5e1)))
                .boxed()
        ]
        .width(Length::Fixed(total_width))
        .height(Length::Fixed(36.0))
        .align_y(alignment::Vertical::Center);

        for (col_idx, header_title) in table.headers.iter().enumerate() {
            if col_idx > 0 {
                let v_sep = container(space())
                    .width(Length::Fixed(1.0))
                    .height(Length::Fixed(36.0))
                    .style(|_theme| container::Style::default().background(color!(0xcbd5e1)));
                header_row = header_row.push(v_sep.boxed());
            }

            let is_editing_this_header =
                state.editing_cell == Some(CellCoord::Header(col_idx));

            let cell_widget: Element<'a, Message> = if is_editing_this_header {
                container(
                    row![
                        text_input("", &state.editing_value)
                            .id(CELL_INPUT_ID)
                            .on_input(Message::EditingValueChanged)
                            .on_submit(Message::SaveCell)
                            .size(11)
                            .padding([3, 5])
                            .width(Length::Fill)
                            .style(|_theme, _status| text_input::Style {
                                background: color!(0xffffff).into(),
                                border: border::rounded(4).color(color!(0x07bc9f)).width(1.5),
                                placeholder: color!(0x94a3b8),
                                value: color!(0x0f172a),
                                selection: color!(0x07bc9f, 0.2),
                            }),
                        space().width(Length::Fixed(2.0)),
                        button(icons::x().size(11).color(color!(0xef4444)))
                            .padding(2)
                            .on_press(Message::CancelEditing)
                            .style(|_theme, status| {
                                let bg = match status {
                                    button::Status::Hovered => Some(color!(0xfee2e2).into()),
                                    _ => None,
                                };
                                button::Style {
                                    background: bg,
                                    border: border::rounded(4),
                                    ..Default::default()
                                }
                            }),
                    ]
                    .align_y(alignment::Vertical::Center),
                )
                .padding([2, 4])
                .align_y(alignment::Vertical::Center)
                .width(Length::Fixed(COL_WIDTH))
                .height(Length::Fixed(36.0))
                .boxed()
            } else {
                let header_label = text(header_title)
                    .size(11)
                    .wrapping(text::Wrapping::None)
                    .ellipsis(text::Ellipsis::End)
                    .font(font::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(color!(0x475569));

                let delete_col_btn = button(icons::x().size(11).color(color!(0x94a3b8)))
                    .padding(2)
                    .on_press(Message::DeleteColumn(col_idx))
                    .style(|_theme, status| {
                        let bg = match status {
                            button::Status::Hovered => Some(color!(0xfee2e2).into()),
                            _ => None,
                        };
                        button::Style {
                            background: bg,
                            border: border::rounded(4),
                            ..Default::default()
                        }
                    });

                container(
                    row![
                        mouse_area(
                            container(header_label)
                                .padding([8, 8])
                                .clip(true)
                                .align_y(alignment::Vertical::Center)
                                .width(Length::Fill)
                                .height(Length::Fixed(36.0)),
                        )
                        .interaction(mouse::Interaction::Pointer)
                        .on_double_click(Message::StartEditingCell(CellCoord::Header(col_idx))),
                        delete_col_btn,
                    ]
                    .align_y(alignment::Vertical::Center)
                    .padding([0, 4]),
                )
                .width(Length::Fixed(COL_WIDTH))
                .height(Length::Fixed(36.0))
                .boxed()
            };

            header_row = header_row.push(cell_widget);
        }

        let header_container = container(header_row)
            .width(Length::Fixed(total_width))
            .height(Length::Fixed(36.0))
            .style(|_theme| {
                container::Style::default().background(color!(0xf8fafc))
            });

        // 2. Data Rows (strictly fixed to 40px each)
        let query = state.search_query.trim().to_lowercase();
        let mut rows_column = column![].width(Length::Fixed(total_width));

        for (row_idx, row_data) in table.rows.iter().enumerate() {
            if !query.is_empty()
                && !row_data.iter().any(|c| c.to_lowercase().contains(&query))
            {
                continue;
            }

            let is_selected = state.is_row_selected(row_idx);

            let row_check_icon = if is_selected {
                icons::check_square().size(15).color(color!(0x07bc9f))
            } else {
                icons::square().size(15).color(color!(0xcbd5e1))
            };

            let row_check_btn = button(
                container(row_check_icon)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            )
            .width(Length::Fixed(SELECT_COL_WIDTH))
            .height(Length::Fixed(40.0))
            .padding(0)
            .on_press(Message::ToggleRowSelection(row_idx))
            .style(|_theme, _status| button::Style {
                background: None,
                border: border::rounded(0),
                ..Default::default()
            });

            let mut row_widget = row![
                row_check_btn.boxed(),
                container(space())
                    .width(Length::Fixed(1.0))
                    .height(Length::Fixed(40.0))
                    .style(|_theme| container::Style::default().background(color!(0xe2e8f0)))
                    .boxed()
            ]
            .width(Length::Fixed(total_width))
            .height(Length::Fixed(40.0))
            .align_y(alignment::Vertical::Center);

            for (col_idx, cell_value) in row_data.iter().enumerate() {
                if col_idx > 0 {
                    let v_sep = container(space())
                        .width(Length::Fixed(1.0))
                        .height(Length::Fixed(40.0))
                        .style(|_theme| container::Style::default().background(color!(0xe2e8f0)));
                    row_widget = row_widget.push(v_sep.boxed());
                }

                let is_editing = state.editing_cell
                    == Some(CellCoord::Cell {
                        row: row_idx,
                        col: col_idx,
                    });

                let cell_el: Element<'a, Message> = if is_editing {
                    container(
                        row![
                            text_input("", &state.editing_value)
                                .id(CELL_INPUT_ID)
                                .on_input(Message::EditingValueChanged)
                                .on_submit(Message::SaveCell)
                                .size(13)
                                .padding([3, 5])
                                .width(Length::Fill)
                                .style(|_theme, _status| text_input::Style {
                                    background: color!(0xffffff).into(),
                                    border: border::rounded(4).color(color!(0x07bc9f)).width(1.5),
                                    placeholder: color!(0x94a3b8),
                                    value: color!(0x0f172a),
                                    selection: color!(0x07bc9f, 0.2),
                                }),
                            space().width(Length::Fixed(2.0)),
                            button(icons::x().size(11).color(color!(0xef4444)))
                                .padding(2)
                                .on_press(Message::CancelEditing)
                                .style(|_theme, status| {
                                    let bg = match status {
                                        button::Status::Hovered => Some(color!(0xfee2e2).into()),
                                        _ => None,
                                    };
                                    button::Style {
                                        background: bg,
                                        border: border::rounded(4),
                                        ..Default::default()
                                    }
                                }),
                        ]
                        .align_y(alignment::Vertical::Center),
                    )
                    .padding([2, 4])
                    .align_y(alignment::Vertical::Center)
                    .width(Length::Fixed(COL_WIDTH))
                    .height(Length::Fixed(40.0))
                    .boxed()
                } else {
                    let text_color = if is_selected {
                        color!(0x064e43)
                    } else if col_idx == 0 {
                        color!(0x1e293b)
                    } else {
                        color!(0x64748b)
                    };

                    let font_weight = if is_selected || col_idx == 0 {
                        font::Weight::Bold
                    } else {
                        font::Weight::Normal
                    };

                    let cell_label = text(cell_value)
                        .size(13)
                        .wrapping(text::Wrapping::None)
                        .ellipsis(text::Ellipsis::End)
                        .font(font::Font {
                            weight: font_weight,
                            ..Default::default()
                        })
                        .color(text_color);

                    mouse_area(
                        container(cell_label)
                            .padding([8, 10])
                            .clip(true)
                            .align_y(alignment::Vertical::Center)
                            .width(Length::Fixed(COL_WIDTH))
                            .height(Length::Fixed(40.0)),
                    )
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(Message::SelectRow(row_idx))
                    .on_double_click(Message::StartEditingCell(CellCoord::Cell {
                        row: row_idx,
                        col: col_idx,
                    }))
                    .boxed()
                };

                row_widget = row_widget.push(cell_el);
            }

            let row_bg = if is_selected {
                color!(0x07bc9f, 0.12)
            } else {
                color!(0xffffff)
            };

            let row_container = container(row_widget)
                .width(Length::Fixed(total_width))
                .height(Length::Fixed(40.0))
                .style(move |_theme| {
                    container::Style::default().background(row_bg)
                });

            let row_divider = container(space())
                .width(Length::Fixed(total_width))
                .height(Length::Fixed(1.0))
                .style(|_theme| container::Style::default().background(color!(0xe2e8f0)));

            rows_column = rows_column
                .push(row_container.boxed())
                .push(row_divider.boxed());
        }

        let header_divider = container(space())
            .width(Length::Fixed(total_width))
            .height(Length::Fixed(1.0))
            .style(|_theme| container::Style::default().background(color!(0xcbd5e1)));

        // Table grid: pinned header (36px) + divider (1px) + vertical scrollable rows,
        // wrapped in horizontal scrollable so headers and rows scroll horizontally together.
        let rows_scrollable = scrollable(rows_column)
            .id(TABLE_ROWS_SCROLLABLE_ID)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::default(),
            ))
            .width(Length::Fixed(total_width))
            .height(Length::Fill)
            .style(crate::core::widgets::light_scrollbar_style);

        let grid = column![
            header_container.boxed(),
            header_divider.boxed(),
            rows_scrollable.boxed(),
        ]
        .width(Length::Fixed(total_width))
        .height(Length::Fill);

        scrollable(grid)
            .id(TABLE_SCROLLABLE_ID)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default(),
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::core::widgets::light_scrollbar_style)
            .boxed()
    } else {
        container(
            text("Selecione um dataset acima...")
                .size(13)
                .color(color!(0x94a3b8)),
        )
        .center(Length::Fill)
        .boxed()
    };

    let has_dataset = state.dataset.is_some();

    // Right Column Button: occupies full width and height of the right strip
    let add_column_button = button(
        container(icons::plus().size(15))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press_maybe(has_dataset.then_some(Message::AddColumn))
    .style(|_theme, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (Some(color!(0xe2e8f0).into()), color!(0x07bc9f)),
            button::Status::Pressed => (Some(color!(0xcbd5e1).into()), color!(0x059669)),
            button::Status::Disabled => (Some(color!(0xf8fafc).into()), color!(0xcbd5e1)),
            _ => (Some(color!(0xf8fafc).into()), color!(0x64748b)),
        };
        button::Style {
            background: bg,
            text_color,
            border: border::rounded(0),
            ..button::Style::default()
        }
    });

    let add_column_strip = container(add_column_button)
        .width(Length::Fixed(30.0))
        .height(Length::Fill);

    // Bottom Row Button: occupies full width and height of the bottom strip
    let add_row_button = button(
        container(icons::plus().size(15))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press_maybe(has_dataset.then_some(Message::AddRow))
    .style(|_theme, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (Some(color!(0xe2e8f0).into()), color!(0x07bc9f)),
            button::Status::Pressed => (Some(color!(0xcbd5e1).into()), color!(0x059669)),
            button::Status::Disabled => (Some(color!(0xf8fafc).into()), color!(0xcbd5e1)),
            _ => (Some(color!(0xf8fafc).into()), color!(0x64748b)),
        };
        button::Style {
            background: bg,
            text_color,
            border: border::rounded(0),
            ..button::Style::default()
        }
    });

    let add_row_strip = container(add_row_button)
        .width(Length::Fill)
        .height(Length::Fixed(30.0));

    // Full table area:
    // Left side: table_content + bottom add_row_strip
    // Right side: vertical divider rule + add_column_strip (full height)
    let body_area = row![
        column![
            container(table_content)
                .width(Length::Fill)
                .height(Length::Fill),
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xe2e8f0),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            add_row_strip,
        ]
        .width(Length::Fill)
        .height(Length::Fill),
        rule::vertical(1).style(|_theme| rule::Style {
            color: color!(0xe2e8f0),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }),
        add_column_strip,
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    let table_container = container(
        column![
            search_bar,
            rule::horizontal(1).style(|_theme| rule::Style {
                color: color!(0xe2e8f0),
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }),
            body_area,
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xffffff))
            .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
    });

    let mut outer_col = column![table_container.boxed()]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill);

    if !state.selected_rows.is_empty() {
        let count = state.selected_count();
        let delete_rows_btn = button(
            row![
                icons::trash().size(11).color(color!(0xdc2626)),
                space().width(Length::Fixed(4.0)),
                text(format!("Excluir selecionado(s) ({count})"))
                    .size(11)
                    .color(color!(0xb91c1c)),
            ]
            .align_y(alignment::Vertical::Center),
        )
        .padding([3, 8])
        .on_press(Message::DeleteSelectedRows)
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered => color!(0xfecaca),
                button::Status::Pressed => color!(0xfca5a5),
                _ => color!(0xfee2e2),
            };
            button::Style {
                background: Some(bg.into()),
                border: border::rounded(12).color(color!(0xfca5a5)).width(1.0),
                ..Default::default()
            }
        });

        let badge = button(
            row![
                text(format!("{count} selecionado(s)")).size(11),
                icons::x().size(11),
            ]
            .spacing(6)
            .align_y(alignment::Vertical::Center),
        )
        .padding([4, 10])
        .on_press(Message::ClearSelection)
        .style(|_theme, status| {
            let (bg, border_color, text_color) = match status {
                button::Status::Hovered => (color!(0xbbf7d0), color!(0x07bc9f), color!(0x064e3b)),
                button::Status::Pressed => (color!(0x86efac), color!(0x059669), color!(0x064e3b)),
                _ => (color!(0xd1fae5), color!(0xa7f3d0), color!(0x065f46)),
            };
            button::Style {
                background: Some(bg.into()),
                text_color,
                border: border::rounded(12).color(border_color).width(1.0),
                ..Default::default()
            }
        });

        let selection_footer = container(
            row![badge, space().width(Length::Fill), delete_rows_btn]
                .align_y(alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding([2, 2]);

        outer_col = outer_col.push(selection_footer.boxed());
    }

    outer_col
}
