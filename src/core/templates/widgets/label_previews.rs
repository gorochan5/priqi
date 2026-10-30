use iced::widget::{column, container, row, rule, space, text};
use iced::{Element, Length, Widget, border, color, font};

use crate::core::icons;
use crate::core::templates::models::{Message, SampleTemplateKind};

pub fn render_preview<'a>(kind: SampleTemplateKind) -> Element<'a, Message> {
    match kind {
        SampleTemplateKind::LocalEstoque => preview_local_estoque().boxed(),
        SampleTemplateKind::ProdutoAcabado => preview_produto_acabado().boxed(),
        SampleTemplateKind::MateriaPrima => preview_materia_prima().boxed(),
        SampleTemplateKind::Custom => preview_custom().boxed(),
    }
}

fn preview_custom() -> impl Widget<Message> {
    let barcode_bars = [
        3.0, 1.0, 2.0, 1.5, 3.5, 1.0, 2.0, 1.0, 4.0, 1.5, 2.5, 1.0, 3.0, 2.0, 1.5, 3.5, 1.0,
        2.0, 1.5, 3.0,
    ];

    let mut barcode = row![].spacing(1.5);
    for &w in &barcode_bars {
        barcode = barcode.push(
            container(space())
                .width(Length::Fixed(w))
                .height(Length::Fixed(32.0))
                .style(|_theme| container::Style::default().background(color!(0x475569))),
        );
    }

    container(
        column![
            barcode,
            space().height(Length::Fixed(8.0)),
            text("[NOVA ETIQUETA]")
                .size(10)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x475569)),
        ]
        .align_x(iced::alignment::Horizontal::Center),
    )
    .width(Length::Fixed(180.0))
    .height(Length::Fixed(90.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xffffff))
            .border(border::rounded(6).color(color!(0x64748b)).width(1.0))
    })
}

fn preview_local_estoque() -> impl Widget<Message> {
    let barcode_bars = [
        2.5, 1.0, 3.5, 1.0, 1.5, 4.0, 2.0, 1.0, 3.0, 1.5, 2.0, 1.0, 4.0, 1.0, 2.5, 3.5, 1.0,
        2.0, 1.5, 3.5, 2.0, 1.0, 3.0, 1.0, 2.0, 4.0, 1.0, 3.0, 2.0, 1.5,
    ];

    let mut barcode = row![].spacing(1.5);
    for &w in &barcode_bars {
        barcode = barcode.push(
            container(space())
                .width(Length::Fixed(w))
                .height(Length::Fixed(38.0))
                .style(|_theme| container::Style::default().background(color!(0x0f172a))),
        );
    }

    container(
        column![
            barcode,
            space().height(Length::Fixed(8.0)),
            text("[ETI_LOCEST]")
                .size(11)
                .font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .color(color!(0x0f172a)),
        ]
        .align_x(iced::alignment::Horizontal::Center),
    )
    .width(Length::Fixed(180.0))
    .height(Length::Fixed(90.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme| {
        container::Style::default()
            .background(color!(0xffffff))
            .border(border::rounded(6).color(color!(0x334155)).width(1.0))
    })
}

fn preview_produto_acabado() -> impl Widget<Message> {
    render_kronos_label(false)
}

fn preview_materia_prima() -> impl Widget<Message> {
    render_kronos_label(true)
}

fn render_kronos_label(is_raw_material: bool) -> impl Widget<Message> {
    // Header
    let header = row![
        row![
            icons::gem().size(9).color(color!(0x0f172a)),
            space().width(Length::Fixed(2.0)),
            text("Kronos Pet").size(8).font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            }),
        ]
        .align_y(iced::alignment::Vertical::Center),
        space().width(Length::Fill),
        column![
            text("ANIMUS PET LTDA").size(4.5).color(color!(0x475569)),
            text("CNPJ: 29.101.995/0001-46").size(4.0).color(color!(0x64748b)),
            text("Tel: (19) 3935-8925").size(4.0).color(color!(0x64748b)),
        ],
    ]
    .align_y(iced::alignment::Vertical::Center);

    // Vertical barcode for right side
    let v_bars = [
        1.5, 2.5, 1.0, 3.0, 1.0, 1.5, 2.0, 1.0, 3.5, 2.0, 1.0, 1.5, 2.0, 1.0, 3.0, 1.5, 2.0,
        1.0, 1.5, 2.5, 1.0,
    ];
    let mut vertical_barcode = column![].spacing(1.0);
    for &h in &v_bars {
        vertical_barcode = vertical_barcode.push(
            container(space())
                .height(Length::Fixed(h))
                .width(Length::Fixed(22.0))
                .style(|_theme| container::Style::default().background(color!(0x0f172a))),
        );
    }

    let code_size = if is_raw_material { 10.0 } else { 7.5 };

    let middle = row![
        column![
            text("Cliente: [EMP_ERAZAO]").size(5.0).color(color!(0x475569)),
            space().height(Length::Fixed(1.0)),
            text("Produto:").size(4.5).color(color!(0x64748b)),
            text("[PRO_CODPRO]").size(code_size).font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            }),
            text("[PRO_DESCR]").size(4.5).color(color!(0x64748b)),
        ]
        .width(Length::Fill),
        rule::vertical(1).style(|_theme| rule::Style {
            color: color!(0xcbd5e1),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }),
        space().width(Length::Fixed(2.0)),
        vertical_barcode,
    ]
    .align_y(iced::alignment::Vertical::Center);

    // Table rows
    let table_cell = |title: &'static str, val: &'static str| {
        container(
            column![
                text(title).size(4.0).color(color!(0x64748b)),
                text(val).size(5.0).font(font::Font {
                    weight: font::Weight::Medium,
                    ..Default::default()
                }),
            ]
            .padding(1),
        )
        .width(Length::Fill)
        .style(|_theme| {
            container::Style::default()
                .border(border::rounded(0).color(color!(0xe2e8f0)).width(0.5))
        })
    };

    let table_row_1 = row![
        table_cell("Fab:", "[ETI_COMP04]"),
        table_cell("Lote:", "[ETI_LOTEFG]"),
        table_cell("P. Bruto:", "[ETI_PESOBR]"),
    ];

    let table_row_2 = row![
        table_cell("Val:", "[ETI_COMP05]"),
        table_cell("Qtd:", "[ETI_QTDENV]"),
        table_cell("P. Líq:", "[ETI_PESOLQ]"),
    ];

    let divider = || {
        rule::horizontal(1).style(|_theme| rule::Style {
            color: color!(0xcbd5e1),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        })
    };

    let mut body = column![
        header,
        divider(),
        middle,
        divider(),
        table_row_1,
        table_row_2,
    ]
    .spacing(2);

    if !is_raw_material {
        let table_row_3 = row![
            table_cell("Operador:", "[ETI_OPERAD]"),
            table_cell("Custo:", "[ETI_CUSTOU]"),
            table_cell("Cód:", "[ETI_CODIOF]"),
        ];
        body = body.push(table_row_3.boxed());
    }

    container(body)
        .width(Length::Fixed(190.0))
        .height(Length::Fixed(105.0))
        .padding(4)
        .style(|_theme| {
            container::Style::default()
                .background(color!(0xffffff))
                .border(border::rounded(4).color(color!(0x334155)).width(1.0))
        })
}
