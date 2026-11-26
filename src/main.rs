#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
                    let dataset_meta = db.load_dataset_metadata().await.unwrap_or_default();
                    let initial_table = if let Some((first_id, _)) = dataset_meta.first() {
                        db.load_dataset_table(first_id)
                            .await
                            .ok()
                            .flatten()
                            .map(|t| (first_id.clone(), t))
                    } else {
                        None
                    };
                    Ok((Arc::new(db), templates, dataset_meta, initial_table))
                }
                Err(e) => Err(format!("Erro ao inicializar banco SQLite: {e}")),
            }
        },
        Message::DbInitialized,
    );

    (state, init_task)
}

struct SmallTokioExecutor(tokio::runtime::Runtime);

impl iced::executor::Executor for SmallTokioExecutor {
    fn new() -> Result<Self, std::io::Error> {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .max_blocking_threads(2)
            .thread_stack_size(512 * 1024)
            .enable_all()
            .build()
            .map(Self)
    }

    fn spawn(&self, future: impl std::future::Future<Output = ()> + Send + 'static) {
        drop(self.0.spawn(future));
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = self.0.enter();
        f()
    }

    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        self.0.block_on(future)
    }
}

fn subscription(_state: &State) -> iced::Subscription<Message> {
    let close_sub = iced::window::close_requests().map(Message::CloseRequested);

    let events_sub = iced::event::listen_with(|event, status, id| match (event, status) {
        (
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
                ..
            }),
            _,
        ) => Some(Message::EscapePressed),
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

    iced::Subscription::batch([close_sub, events_sub])
}

pub fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("priqi=info,warn")),
        )
        .init();

    let icon = image::load_from_memory(include_bytes!("../resources/icon.png"))
        .ok()
        .and_then(|img| {
            let rgba = img.into_rgba8();
            let (width, height) = rgba.dimensions();
            iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
        });

    iced::application(boot, update, view)
        .executor::<SmallTokioExecutor>()
        .backend(iced::Backend::Software)
        .antialiasing(false)
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
