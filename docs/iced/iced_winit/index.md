---
title: "iced_winit"
kind: crate
path: "iced_winit"
crate: "iced_winit"
file: "iced_winit/index.md"
docs_url: "https://docs.iced.rs/iced_winit/index.html"
source_file: "src/iced_winit/lib.rs"
source_lines: "1-1952"
---

# `iced_winit`

A windowing shell for Iced, on top of [`winit`](https://github.com/rust-windowing/winit).

![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)

`iced_winit` offers some convenient abstractions on top of [`iced_runtime`](https://github.com/iced-rs/iced/tree/master/runtime) to quickstart development when using [`winit`](https://github.com/rust-windowing/winit).

It exposes a renderer-agnostic [`Program`] trait that can be implemented and then run with a simple call. The use of this trait is optional.

Additionally, a [`conversion`](conversion/index.md) module is available for users that decide to implement a custom event loop.

## Re-exports

- [`Clipboard`](clipboard/struct.Clipboard.md)
- `pub use iced_debug as debug;`
- `pub use iced_program as program;`
- `pub use iced_runtime as runtime;`
- `pub use program::core;`
- `pub use program::graphics;`
- `pub use runtime::futures;`
- `pub use winit;`

## Modules

- [`clipboard`](clipboard/index.md) — Access the clipboard.
- [`conversion`](conversion/index.md) — Convert [`winit`](https://github.com/rust-windowing/winit) types into [`iced_runtime`](https://github.com/iced-rs/iced/tree/master/runtime) types, and viceversa.

## Structs

- [`Proxy`](struct.Proxy.md) — An event loop proxy with backpressure that implements `Sink`.

## Enums

- [`Error`](enum.Error.md) — An error that occurred while running an application.

## Functions

- [`build_user_interfaces`](fn.build_user_interfaces.md) — Build the user interface for every window.
- [`run`](fn.run.md) — Runs a [`Program`] with the provided settings.
- [`user_force_quit`](fn.user_force_quit.md) — Returns true if the provided event should cause a [`Program`] to exit.
