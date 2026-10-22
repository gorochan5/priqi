---
title: "task"
kind: module
path: "iced::task"
crate: "iced"
module: "iced"
file: "iced/task/index.md"
docs_url: "https://docs.iced.rs/iced/task/index.html"
source_file: "src/iced/lib.rs"
source_lines: "556"
---

# `iced::task`

Create runtime tasks.

## Structs

- [`Handle`](struct.Handle.md) — A handle to a [`Task`](../struct.Task.md) that can be used for aborting it.
- [`Task`](struct.Task.md) — A set of concurrent actions to be performed by the iced runtime.

## Traits

- [`Sipper`](trait.Sipper.md) *(sipper)* — A sipper is both a [`Stream`] that produces a bunch of progress and a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) that produces some final output.
- [`Straw`](trait.Straw.md) *(sipper)* — A [`Straw`](trait.Straw.md) is a [`Sipper`](trait.Sipper.md) that can fail.

## Functions

- [`sipper`](fn.sipper.md) *(sipper)* — Creates a new [`Sipper`](trait.Sipper.md) from the given async closure, which receives a [`Sender`] that can be used to notify progress asynchronously.
- [`stream`](fn.stream.md) *(sipper)* — Turns a [`Sipper`](trait.Sipper.md) into a [`Stream`].

## Type Aliases

- [`Never`](type.Never.md) *(sipper)* — A type with no possible values.
