---
title: "src/iced_winit/error.rs"
kind: source
path: "src/iced_winit/error.rs"
crate: "iced_winit"
file: "src/iced_winit/error.rs.md"
docs_url: "https://docs.iced.rs/src/iced_winit/error.rs.html"
source_file: "src/iced_winit/error.rs"
---

# `src/iced_winit/error.rs`

```rust
use crate::core::backend;
use crate::futures::futures;

/// An error that occurred while running an application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed(winit::error::OsError),

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(backend::Error),
}

impl From<backend::Error> for Error {
    fn from(error: backend::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}
```
