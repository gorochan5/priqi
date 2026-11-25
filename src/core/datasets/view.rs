use iced::widget::{button, column, container, mouse_area, row, rule, space, stack, text, text_input};
use iced::{Length, Padding, Shadow, Vector, Widget, alignment, border, color};

use crate::core::datasets::models::{DATASET_RENAME_INPUT_ID, Message, State};
use crate::core::datasets::widgets::dataset_table::dataset_table;
use crate::core::datasets::widgets::datasets_dropdown::{datasets_dropdown, datasets_dropdown_menu};
use crate::core::icons::{check, database, plus, x};
use crate::core::widgets::import_modal::upload_button;

pub fn view<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    datasets(state)
}

fn datasets<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    let selector_area: iced::Element<'a, Message> = if state.renaming_dataset_id.is_some() {
        row![
            text_input("Nome do dataset", &state.dataset_name_input)
                .id(DATASET_RENAME_INPUT_ID)
                .on_input(Message::DatasetNameChanged)
                .on_submit(Message::SaveDatasetRename)
                .size(13)
                .padding([5, 8])
                .width(Length::Fill)
                .style(|_theme, _status| text_input::Style {
                    background: color!(0xffffff).into(),
                    border: border::rounded(8).color(color!(0x07bc9f)).width(1.5),
                    placeholder: color!(0x94a3b8),
                    value: color!(0x0f172a),
                    selection: color!(0x07bc9f, 0.2),
                }),
            space().width(Length::Fixed(4.0)),
            button(
                container(check().size(14).color(color!(0x07bc9f)))
                    .center_x(Length::Shrink)
                    .center_y(Length::Shrink),
            )
            .padding(4)
            .on_press(Message::SaveDatasetRename)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Some(color!(0xe8f6f3).into()),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: border::rounded(6),
                    ..button::Style::default()
                }
            }),
            space().width(Length::Fixed(2.0)),
            button(
                container(x().size(14).color(color!(0xef4444)))
                    .center_x(Length::Shrink)
                    .center_y(Length::Shrink),
            )
            .padding(4)
            .on_press(Message::CancelDatasetRename)
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Some(color!(0xfee2e2).into()),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: border::rounded(6),
                    ..button::Style::default()
                }
            }),
        ]
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center)
        .boxed()
    } else {
        container(datasets_dropdown(state)).width(Length::Fill).boxed()
    };

    let body: iced::Element<'a, Message> = if state.is_dropdown_open && !state.options.is_empty() {
        stack![
            dataset_table(state).boxed(),
            mouse_area(space().width(Length::Fill).height(Length::Fill))
                .on_press(Message::CloseDatasetDropdown)
                .boxed(),
            container(datasets_dropdown_menu(state))
                .width(Length::Fill)
                .padding(Padding {
                    top: 0.0,
                    right: 4.0,
                    bottom: 0.0,
                    left: 4.0,
                })
                .boxed(),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .boxed()
    } else {
        dataset_table(state).boxed()
    };

    container(
        column![
            row![
                database().size(24).color(color!(0x07bc9f)),
                space().width(Length::Fixed(5.0)),
                text("DATASETS").size(14).color(color!(0x64748b)),
                space().width(Length::Fixed(10.0)),
                selector_area,
                space().width(Length::Fixed(6.0)),
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
            body,
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

fn add_button<'a>() -> impl Widget<Message> + 'a {
    button(
        container(plus().size(18).color(color!(0x07bc9f)))
            .center_x(Length::Shrink)
            .center_y(Length::Shrink),
    )
    .padding(4)
    .on_press(Message::AddDataset)
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
