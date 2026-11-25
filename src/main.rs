use std::sync::Arc;

use crate::core::{
    models::{Message, State},
    update::update,
    view::view,
};

mod core;

fn boot() -> (State, iced::Task<Message>) {
    let state = State::default();
    let init_task = iced::Task::perform(
        async {
            match crate::core::db::Db::init().await {
                Ok(db) => {
                    let templates = db.load_templates().await.unwrap_or_default();
                    let datasets = db.load_datasets().await.unwrap_or_default();
                    Ok((Arc::new(db), templates, datasets))
                }
                Err(e) => Err(format!("Erro ao inicializar banco SQLite: {e}")),
            }
        },
        Message::DbInitialized,
    );

    (state, init_task)
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    let close_sub = iced::window::close_requests().map(Message::CloseRequested);

    let keyboard_sub = iced::keyboard::listen().map(|event| match event {
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
            ..
        } => Message::EscapePressed,
        _ => Message::Nothing,
    });

    let events_sub = iced::event::listen_with(|event, status, id| match (event, status) {
        (iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)), _) => {
            Some(Message::ModifiersChanged(modifiers))
        }
        (iced::Event::Window(iced::window::Event::FileHovered(_)), _) => {
            Some(Message::FileHovered(true))
        }
        (iced::Event::Window(iced::window::Event::FilesHoveredLeft), _) => {
            Some(Message::FileHovered(false))
        }
        (iced::Event::Window(iced::window::Event::FileDropped(path)), _) => {
            Some(Message::FileDropped(path))
        }
        (iced::Event::Window(iced::window::Event::Resized(_)), _) => {
            Some(Message::WindowResized(id))
        }
        (
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),
            iced::event::Status::Ignored,
        )
        | (iced::Event::Window(iced::window::Event::Unfocused), _) => Some(Message::Editor(
            crate::core::editor::models::Message::Unfocus,
        )),
        _ => None,
    });

    iced::Subscription::batch([close_sub, keyboard_sub, events_sub])
}

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    let icon = iced::window::icon::from_file_data(
        include_bytes!("../resources/icon.png"),
        None,
    )
    .ok();

    iced::application(boot, update, view)
        .subscription(subscription)
        .window(iced::window::Settings {
            icon,
            maximized: true,
            resizable: true,
            decorations: false,
            exit_on_close_request: false,
            ..Default::default()
        })
        .fonts([core::icons::FONT_BYTES])
        .centered()
        .title("Priqi — Impressão de Etiquetas")
        .run()
}
