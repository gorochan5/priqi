---
title: "task"
kind: module
path: "iced_runtime::task"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/task/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/task/index.html"
source_file: "src/iced_runtime/task.rs"
source_lines: "1-536"
---

# `iced_runtime::task`

Create runtime tasks.

## Re-exports

- `pub use sipper::Never;sipper` *(sipper)*
- `pub use sipper::Sender;sipper` *(sipper)*
- `pub use sipper::Sipper;sipper` *(sipper)*
- `pub use sipper::Straw;sipper` *(sipper)*
- `pub use sipper::sipper;sipper` *(sipper)*
- `pub use sipper::stream;sipper` *(sipper)*

## Structs

- [`Handle`](struct.Handle.md) — A handle to a [`Task`](struct.Task.md) that can be used for aborting it.
- [`Task`](struct.Task.md) — A set of concurrent actions to be performed by the iced runtime.

## Functions

- [`blocking`](fn.blocking.md) — Creates a new [`Task`](struct.Task.md) that will run the given closure in a new thread.
- [`channel`](fn.channel.md) — Creates a new [`Task`](struct.Task.md) that executes the [`Action`](../enum.Action.md) returned by the closure and produces the values fed to the [`mpsc::Sender`].
- [`effect`](fn.effect.md) — Creates a new [`Task`](struct.Task.md) that executes the given [`Action`](../enum.Action.md) and produces no output.
- [`into_stream`](fn.into_stream.md) — Returns the underlying [`Stream`] of the [`Task`](struct.Task.md).
- [`oneshot`](fn.oneshot.md) — Creates a new [`Task`](struct.Task.md) that executes the [`Action`](../enum.Action.md) returned by the closure and produces the value fed to the [`oneshot::Sender`].
- [`try_blocking`](fn.try_blocking.md) — Creates a new [`Task`](struct.Task.md) that will run the given closure that can fail in a new thread.
- [`widget`](fn.widget.md) — Creates a new [`Task`](struct.Task.md) that runs the given [`widget::Operation`] and produces its output.
