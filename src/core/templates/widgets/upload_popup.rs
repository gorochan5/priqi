use iced::widget::{button, center, column, container, mouse_area, opaque, row, space, text};
use iced::{Element, Font, Length, Shadow, Vector, Widget, alignment, border, color, font};

use crate::core::icons::arrow_down_to_line;
use crate::core::templates::models::{Message, State, UploadTab};
use crate::core::templates::widgets::dropzone::dropzone;
use crate::core::templates::widgets::paste_editor::paste_editor;
use crate::core::templates::widgets::tab_button::tab_button;

pub fn upload_popup(state: &State) -> impl Widget<Message> {
    // 1. Título "Importar Etiqueta"
    let title = text("Importar Etiqueta")
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
                UploadTab::Paste,
                state.upload_tab == UploadTab::Paste,
                true,
            ),
            container(space().width(Length::Fixed(1.0)).height(Length::Fixed(18.0)))
                .style(|_theme| container::Style::default().background(color!(0xcbd5e1))),
            tab_button(
                "Importar arquivo",
                UploadTab::File,
                state.upload_tab == UploadTab::File,
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

    // 3. Área Central (Editor ou Dropzone)
    let body_content: Element<'_, Message> = match state.upload_tab {
        UploadTab::Paste => paste_editor(state).boxed(),
        UploadTab::File => dropzone(state).boxed(),
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
        .on_press(Message::CloseUploadModal),
    )
}

// Re-export com o nome upload_modal para retrocompatibilidade
pub use upload_popup as upload_modal;
