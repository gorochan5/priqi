use iced::widget::{
    button, column, container, mouse_area, row, rule, scrollable, space, text, text_input,
};
use iced::{Element, Length, Widget, alignment, border, color, font, mouse};

use crate::core::datasets::models::{CellCoord, Message, State, TABLE_SCROLLABLE_ID};
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
            space().width(Length::Fixed(8.0)),
            icons::sliders().size(15).color(color!(0x64748b)),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding([8, 12])
    .style(|_theme| container::Style::default().background(color!(0xf8fafc)));

    let table_content: Element<'a, Message> = if let Some(table) = state.current_table() {
        let col_count = table.headers.len();
        const COL_WIDTH: f32 = 140.0;
        let total_width =
            4.0 + (col_count as f32) * COL_WIDTH + (col_count.saturating_sub(1) as f32) * 1.0;

        // 1. Headers Row (strictly fixed to 36px)
        let mut header_row = row![
            space().width(Length::Fixed(4.0)).boxed() // aligns with row indicator
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
                    text_input("", &state.editing_value)
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
                    .font(font::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(color!(0x475569));

                mouse_area(
                    container(header_label)
                        .padding([8, 10])
                        .align_y(alignment::Vertical::Center)
                        .width(Length::Fixed(COL_WIDTH))
                        .height(Length::Fixed(36.0)),
                )
                .interaction(mouse::Interaction::Pointer)
                .on_double_click(Message::StartEditingCell(CellCoord::Header(col_idx)))
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

            let is_selected = state.selected_row == Some(row_idx);

            let indicator_bg = if is_selected {
                color!(0x07bc9f)
            } else {
                color!(0x000000, 0.0)
            };

            let indicator = container(space())
                .width(Length::Fixed(4.0))
                .height(Length::Fixed(40.0))
                .style(move |_theme| {
                    container::Style::default().background(indicator_bg)
                });

            let mut row_widget = row![indicator.boxed()]
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
                        text_input("", &state.editing_value)
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
                        .font(font::Font {
                            weight: font_weight,
                            ..Default::default()
                        })
                        .color(text_color);

                    mouse_area(
                        container(cell_label)
                            .padding([8, 10])
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

        // Table grid: header (36px) + divider (1px) + rows
        let grid = column![
            header_container.boxed(),
            header_divider.boxed(),
            rows_column.boxed(),
        ]
        .width(Length::Fixed(total_width));

        scrollable(grid)
            .id(TABLE_SCROLLABLE_ID)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::default(),
                horizontal: scrollable::Scrollbar::default(),
            })
            .width(Length::Fill)
            .height(Length::Fill)
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

    // Right Column Button: occupies full width and height of the right strip
    let add_column_button = button(
        container(icons::plus().size(15))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press(Message::AddColumn)
    .style(|_theme, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (Some(color!(0xe2e8f0).into()), color!(0x07bc9f)),
            button::Status::Pressed => (Some(color!(0xcbd5e1).into()), color!(0x059669)),
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
    .on_press(Message::AddRow)
    .style(|_theme, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => (Some(color!(0xe2e8f0).into()), color!(0x07bc9f)),
            button::Status::Pressed => (Some(color!(0xcbd5e1).into()), color!(0x059669)),
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

    container(
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
    })
}
