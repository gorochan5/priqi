use iced::widget::{button, center, column, container, mouse_area, opaque, row, space, text};
use iced::{Element, Font, Length, Shadow, Vector, Widget, alignment, border, color, font};

use super::models::{ImportConfig, ImportTab, Message, State};
use super::widgets::dropzone::dropzone;
use super::widgets::paste_editor::paste_editor;
use super::widgets::tab_button::tab_button;
pub use super::widgets::upload_button::upload_button;
use crate::core::icons::arrow_down_to_line;

pub fn view<'a>(state: &'a State, config: ImportConfig) -> impl Widget<Message> + 'a {
    // 1. Título dinâmico ("Importar Etiqueta" ou "Importar Base de Dados")
    let title = text(config.title)
        .size(24)
        .font(Font {
            weight: font::Weight::Bold,
            ..Default::default()
        })
        .color(color!(0x0f172a));

    // 2. Segmented control / Tabs: "Colar conteúdo" | "Importar arquivo"
    let tabs = container(
        row![
            tab_button(
                "Colar conteúdo",
                ImportTab::Paste,
                state.active_tab == ImportTab::Paste,
                true,
            ),
            container(space().width(Length::Fixed(1.0)).height(Length::Fixed(18.0)))
                .style(|_theme| container::Style::default().background(color!(0xcbd5e1))),
            tab_button(
                "Importar arquivo",
                ImportTab::File,
                state.active_tab == ImportTab::File,
                false,
            ),
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding(3)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xe2e8f0))
            .border(border::rounded(20))
    });

    // 3. Área Central (Editor parametrizado ou Dropzone parametrizado)
    let body_content: Element<'_, Message> = match state.active_tab {
        ImportTab::Paste => paste_editor(state, config).boxed(),
        ImportTab::File => dropzone(state, config).boxed(),
    };

    // 4. Botão "Importar" no rodapé
    let import_button = button(
        container(
            row![
                arrow_down_to_line().size(18).color(color!(0xffffff)),
                space().width(Length::Fixed(10.0)),
                text("Importar")
                    .size(15)
                    .color(color!(0xffffff))
                    .font(iced::Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    }),
            ]
        )
        .center_x(Length::Fill),
    )
    .width(Length::Fixed(140.0))
    .padding([9, 20])
    .on_press(Message::ImportPressed)
    .style(|_theme, status| {
        let background = match status {
            button::Status::Hovered => color!(0x06967f),
            button::Status::Pressed => color!(0x057a67),
            _ => color!(0x07bc9f),
        };

        button::Style {
            background: Some(background.into()),
            text_color: color!(0xffffff),
            border: border::rounded(10),
            shadow: Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
                color: color!(0x07bc9f, 0.25),
            },
            snap: true,
        }
    });

    let dialog_card = container(
        column![
            // Topo: Título centralizado
            container(title)
                .width(Length::Fill)
                .center_x(Length::Fill),
            space().height(Length::Fixed(16.0)),
            // Abas centralizadas
            container(tabs)
                .width(Length::Fill)
                .center_x(Length::Fill),
            space().height(Length::Fixed(16.0)),
            // Área de Conteúdo
            container(body_content)
                .width(Length::Fill)
                .height(Length::Fixed(320.0)),
            space().height(Length::Fixed(18.0)),
            // Botão Importar centralizado
            container(import_button)
                .width(Length::Fill)
                .center_x(Length::Fill),
        ]
        .width(Length::Fixed(640.0)),
    )
    .padding(24)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xf8fafc))
            .border(border::rounded(16).color(color!(0xe2e8f0)).width(1.5))
            .shadow(Shadow {
                offset: Vector::new(0.0, 12.0),
                blur_radius: 32.0,
                color: color!(0x000000, 0.20),
            })
    });

    opaque(
        mouse_area(
            center(opaque(dialog_card)).style(|_theme| {
                container::Style::default().background(color!(0x000000, 0.45))
            }),
        )
        .on_press(Message::Close),
    )
}
