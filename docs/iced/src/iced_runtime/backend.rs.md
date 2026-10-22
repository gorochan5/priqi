---
title: "src/iced_runtime/backend.rs"
kind: source
path: "src/iced_runtime/backend.rs"
crate: "iced_runtime"
file: "src/iced_runtime/backend.rs.md"
docs_url: "https://docs.iced.rs/src/iced_runtime/backend.rs.html"
source_file: "src/iced_runtime/backend.rs"
---

# `src/iced_runtime/backend.rs`

```rust
//! Configure a [`Backend`](crate::core::Backend) at runtime.
use crate::Task;
use crate::core::backend;
use crate::futures::futures::channel::oneshot;
use crate::task;

/// An backend operation.
#[derive(Debug)]
pub enum Action {
    /// Switches the [`backend::Settings`] of the current application.
    Configure(
        backend::Settings,
        oneshot::Sender<Result<(), backend::Error>>,
    ),
}

/// Returns a [`Task`] that switches the [`backend::Settings`] of the current application.
///
/// This can be leveraged to switch renderers at runtime.
pub fn configure(settings: backend::Settings) -> Task<Result<(), backend::Error>> {
    task::oneshot(|sender| crate::Action::Backend(Action::Configure(settings, sender)))
}
```
