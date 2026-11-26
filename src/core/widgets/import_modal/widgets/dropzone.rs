use iced::widget::{button, center, column, container, space, text};
use iced::{Background, Element, Font, Length, Widget, alignment, border, color, font};

use crate::core::icons::{circle_alert, cloud_upload, file_check};
use crate::core::widgets::import_modal::models::{ImportConfig, ImportKind, Message, State};

fn format_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    if (bytes as f64) >= MB {
        format!("{:.1} MB", bytes as f64 / MB)
    } else if (bytes as f64) >= KB {
        format!("{:.1} KB", bytes as f64 / KB)
    } else {
        format!("{bytes} B")
    }
}

pub fn dropzone<'a>(state: &'a State, config: ImportConfig) -> impl Widget<Message> + 'a {
    let border_color = if state.file_error.is_some() {
        color!(0xef4444)
    } else if state.is_hovered || state.selected_file.is_some() {
        color!(0x07bc9f)
    } else {
        color!(0x94a3b8)
    };

    let inner_content: Element<'a, Message> = if let Some(err) = &state.file_error {
        column![
            circle_alert().size(46).color(color!(0xef4444)),
            space().height(Length::Fixed(12.0)),
            text(err)
                .size(14)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Default::default()
                })
                .color(color!(0xdc2626)),
            space().height(Length::Fixed(6.0)),
            text("Clique para tentar selecionar outro arquivo ou arraste-o até aqui")
                .size(12)
                .color(color!(0x64748b)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    } else if let Some(path) = &state.selected_file {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Arquivo selecionado")
            .to_string();
        let status_text = if state.is_importing {
            "Importando arquivo...".to_string()
        } else if state.kind == Some(ImportKind::Datasets) {
            let size_label = state
                .selected_file_size
                .map(format_file_size)
                .unwrap_or_else(|| "tamanho desconhecido".to_string());
            format!(
                "Arquivo pronto ({size_label}) — clique em \"Importar\" abaixo ou clique aqui para trocar"
            )
        } else {
            let line_count = state.content.line_count();
            format!(
                "Arquivo carregado ({line_count} linhas) — clique em \"Importar\" abaixo ou clique aqui para trocar"
            )
        };

        column![
            file_check().size(48).color(color!(0x07bc9f)),
            space().height(Length::Fixed(12.0)),
            text(file_name)
                .size(15)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x0f172a)),
            space().height(Length::Fixed(4.0)),
            text(status_text)
                .size(12)
                .color(color!(0x06967f)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    } else {
        let icon_color = if state.is_hovered {
            color!(0x07bc9f)
        } else {
            color!(0x64748b)
        };
        let prompt_text = if state.is_hovered {
            "Solte o arquivo aqui para carregar"
        } else {
            "Clique para selecionar ou arraste um arquivo até aqui"
        };

        column![
            cloud_upload().size(48).color(icon_color),
            space().height(Length::Fixed(12.0)),
            text(prompt_text)
                .size(14)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Default::default()
                })
                .color(color!(0x334155)),
            space().height(Length::Fixed(4.0)),
            text(config.dropzone_hint)
                .size(12)
                .color(color!(0x94a3b8)),
        ]
        .align_x(alignment::Horizontal::Center)
        .boxed()
    };

    let is_hovered = state.is_hovered;
    let has_file = state.selected_file.is_some();

    button(
        container(
            center(inner_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(16),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .on_press_maybe((!state.is_importing).then_some(Message::OpenFilePicker(config.kind)))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(0)
    .style(move |_theme, status| {
        let bg = if is_hovered {
            color!(0xe8f6f3)
        } else if has_file {
            match status {
                button::Status::Hovered => color!(0xe8f6f3),
                button::Status::Pressed => color!(0xdcf2ee),
                _ => color!(0xf0fdf9),
            }
        } else {
            match status {
                button::Status::Hovered => color!(0xf1f5f9),
                button::Status::Pressed => color!(0xe2e8f0),
                _ => color!(0xf8fafc),
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            border: border::rounded(12).color(border_color).width(2.0),
            ..Default::default()
        }
    })
}
