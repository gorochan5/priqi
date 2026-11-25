use iced::widget::{button, column, container, row, scrollable, space, text};
use iced::{Length, Shadow, Vector, Widget, alignment, border, color};

use crate::core::datasets::models::{DatasetOption, Message, State};
use crate::core::icons::{chevron_down, chevron_up, pencil, trash};

pub fn datasets_dropdown<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    let is_open = state.is_dropdown_open && !state.options.is_empty();
    let has_options = !state.options.is_empty();

    let (label, label_color) = match &state.dataset {
        Some(d) => (d.name.as_str(), color!(0x1e293b)),
        None => ("Selecione um dataset...", color!(0x94a3b8)),
    };

    let chevron = if is_open {
        chevron_up().size(14).color(color!(0x07bc9f))
    } else {
        chevron_down().size(14).color(color!(0x07bc9f))
    };

    button(
        row![
            text(label)
                .size(13)
                .color(label_color)
                .ellipsis(text::Ellipsis::End)
                .width(Length::Fill),
            space().width(Length::Fixed(6.0)),
            chevron,
        ]
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .padding([6, 10])
    .on_press_maybe(has_options.then_some(Message::ToggleDatasetDropdown))
    .style(move |_theme, status| {
        let border_color = if is_open || matches!(status, button::Status::Hovered) {
            color!(0x07bc9f)
        } else {
            color!(0xe2e8f0)
        };
        button::Style {
            background: Some(color!(0xffffff).into()),
            text_color: color!(0x1e293b),
            border: border::rounded(8).color(border_color).width(1.0),
            shadow: Shadow::default(),
            snap: true,
        }
    })
}

pub fn datasets_dropdown_menu<'a>(state: &'a State) -> impl Widget<Message> + 'a {
    let active_id = state.dataset.as_ref().map(|d| d.id.as_str());

    let items = state
        .options
        .iter()
        .map(|opt| dropdown_row(opt, active_id == Some(opt.id.as_str())));

    let list = column(items).spacing(2).width(Length::Fill);
    let menu_height = ((state.options.len() as f32) * 32.0 + 12.0).min(240.0);

    container(
        scrollable(list)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::core::widgets::light_scrollbar_style),
    )
    .width(Length::Fill)
    .height(Length::Fixed(menu_height))
    .padding(4)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xffffff))
            .border(border::rounded(8).color(color!(0xe2e8f0)).width(1.0))
            .shadow(Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
                color: color!(0x000000, 0.12),
            })
    })
}

fn dropdown_row<'a>(opt: &'a DatasetOption, is_active: bool) -> iced::Element<'a, Message> {
    let text_color = if is_active {
        color!(0x065f46)
    } else {
        color!(0x1e293b)
    };

    let name_btn = button(
        text(opt.name.as_str())
            .size(13)
            .color(text_color)
            .ellipsis(text::Ellipsis::End)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([5, 8])
    .on_press(Message::DatasetSelected(opt.clone()))
    .style(move |_theme, status| {
        let bg = if is_active {
            Some(color!(0xe8f6f3).into())
        } else if matches!(status, button::Status::Hovered) {
            Some(color!(0xf1f5f9).into())
        } else {
            None
        };
        button::Style {
            background: bg,
            text_color,
            border: border::rounded(6),
            ..button::Style::default()
        }
    });

    let pencil_btn = button(
        container(pencil().size(13))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0))
    .padding(0)
    .on_press(Message::StartRenameDataset(opt.id.clone()))
    .style(|_theme, status| {
        let (bg, icon_color) = match status {
            button::Status::Hovered => (Some(color!(0xd1fae5).into()), color!(0x07bc9f)),
            button::Status::Pressed => (Some(color!(0xa7f3d0).into()), color!(0x065f46)),
            _ => (None, color!(0x64748b)),
        };
        button::Style {
            background: bg,
            text_color: icon_color,
            border: border::rounded(6),
            ..button::Style::default()
        }
    });

    let trash_btn = button(
        container(trash().size(13))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0))
    .padding(0)
    .on_press(Message::DeleteDataset(opt.id.clone()))
    .style(|_theme, status| {
        let (bg, icon_color) = match status {
            button::Status::Hovered => (Some(color!(0xfee2e2).into()), color!(0xdc2626)),
            button::Status::Pressed => (Some(color!(0xfecaca).into()), color!(0xdc2626)),
            _ => (None, color!(0x64748b)),
        };
        button::Style {
            background: bg,
            text_color: icon_color,
            border: border::rounded(6),
            ..button::Style::default()
        }
    });

    container(
        row![name_btn, pencil_btn, trash_btn]
            .spacing(2)
            .width(Length::Fill)
            .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme| {
        if is_active {
            container::Style::default()
                .background(color!(0xe8f6f3))
                .border(border::rounded(6))
        } else {
            container::Style::default()
        }
    })
    .boxed()
}
