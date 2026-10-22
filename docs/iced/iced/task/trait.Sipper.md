---
title: "Sipper"
kind: trait
path: "iced::task::Sipper"
crate: "iced"
module: "iced::task"
file: "iced/task/trait.Sipper.md"
docs_url: "https://docs.iced.rs/iced/task/trait.Sipper.html"
---

# `iced::task::Sipper`

```rust
pub trait Sipper<Output, Progress = Output>: Core<Output = Output, Item = Progress> {
    // Provided methods
    fn with<F, A>(self, f: F) -> With<Self, F, A>
       where Self: Sized,
             F: FnMut(Progress) -> A { ... }
    fn filter_with<F, A>(self, f: F) -> FilterWith<Self, F, A>
       where Self: Sized,
             F: FnMut(Progress) -> Option<A> { ... }
    fn sip(&mut self) -> Next<'_, Self>
       where Self: Unpin { ... }
    fn run<S>(self, on_progress: impl Into<Sender<Progress, S>>) -> Run<Self, S>
       where Self: Sized,
             S: Sink<Progress> { ... }
    fn pin(self) -> Pin<Box<Self>>
       where Self: Sized { ... }
}
```

A sipper is both a [`Stream`] that produces a bunch of progress and a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) that produces some final output.

## Provided Methods

### `with`

```rust
fn with<F, A>(self, f: F) -> With<Self, F, A> where Self: Sized, F: FnMut(Progress) -> A,
```

Maps the progress of the [`Sipper`](trait.Sipper.md) with the given closure.

This is analogous to `map` in many other types; but we use `with` to avoid naming collisions with [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) and [`Stream`].

### `filter_with`

```rust
fn filter_with<F, A>(self, f: F) -> FilterWith<Self, F, A> where Self: Sized, F: FnMut(Progress) -> Option<A>,
```

Maps and filters the progress of the [`Sipper`](trait.Sipper.md) with the given closure.

This is analogous to `filter_map` in many other types; but we use `filter_with` to avoid naming collisions with [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) and [`Stream`].

### `sip`

```rust
fn sip(&mut self) -> Next<'_, Self> where Self: Unpin,
```

Returns the next progress, if any.

When this method returns `None`, it means there is no more progress to be made; and the output is ready.

### `run`

```rust
fn run<S>(self, on_progress: impl Into<Sender<Progress, S>>) -> Run<Self, S> where Self: Sized, S: Sink<Progress>,
```

Runs the [`Sipper`](trait.Sipper.md), sending any progress through the given [`Sender`] and returning its output at the end.

### `pin`

```rust
fn pin(self) -> Pin<Box<Self>> where Self: Sized,
```

Pins the [`Sipper`](trait.Sipper.md) in a [`Box`](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html).

You may need to call this method before being able to [`sip`](trait.Sipper.md).

## Implementors

- `impl<T, Output, Progress> Sipper<Output, Progress> for T where T: Core<Output = Output, Item = Progress>,`
