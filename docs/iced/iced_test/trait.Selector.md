---
title: "Selector"
kind: trait
path: "iced_test::Selector"
crate: "iced_test"
module: "iced_test"
file: "iced_test/trait.Selector.md"
docs_url: "https://docs.iced.rs/iced_test/trait.Selector.html"
---

# `iced_test::Selector`

```rust
pub trait Selector {
    type Output;

    // Required methods
    fn select(&mut self, candidate: Candidate<'_>) -> Option<Self::Output>;
    fn description(&self) -> String;

    // Provided methods
    fn find(self) -> Finder<One<Self>>
       where Self: Sized { ... }
    fn find_all(self) -> Finder<All<Self>>
       where Self: Sized { ... }
}
```

A type that traverses the widget tree to “select” data and produce some output.

## Required Associated Types

### `Output`

```rust
type Output
```

The output type of the [`Selector`](trait.Selector.md).

For most selectors, this will normally be a [`Target`]. However, some selectors may want to return a more limited type to encode the selection guarantees in the type system.

For instance, the implementations of [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html) and [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html) of [`Selector`](trait.Selector.md) return a [`target::Text`] instead of a generic [`Target`], since they are guaranteed to only select text.

## Required Methods

### `select`

```rust
fn select(&mut self, candidate: Candidate<'_>) -> Option<Self::Output>
```

Performs a selection of the given [`Candidate`], if applicable.

This method traverses the widget tree in depth-first order.

### `description`

```rust
fn description(&self) -> String
```

Returns a short description of the [`Selector`](trait.Selector.md) for debugging purposes.

## Provided Methods

### `find`

```rust
fn find(self) -> Finder<One<Self>> where Self: Sized,
```

Returns a [`widget::Operation`] that runs the [`Selector`](trait.Selector.md) and stops after the first [`Output`](trait.Selector.md) is produced.

### `find_all`

```rust
fn find_all(self) -> Finder<All<Self>> where Self: Sized,
```

Returns a [`widget::Operation`] that runs the [`Selector`](trait.Selector.md) for the entire widget tree and aggregates all of its [`Output`](trait.Selector.md).

## Implementors

- `impl<F, T> Selector for F where F: FnMut(Candidate<'_>) -> Option<T>,`
  - `type Output = T`
