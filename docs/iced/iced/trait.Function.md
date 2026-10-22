---
title: "Function"
kind: trait
path: "iced::Function"
crate: "iced"
module: "iced"
file: "iced/trait.Function.md"
docs_url: "https://docs.iced.rs/iced/trait.Function.html"
---

# `iced::Function`

```rust
pub trait Function<A, B, O> {
    // Required method
    fn with(self, prefix: A) -> impl Fn(B);
}
```

A trait extension for binary functions (`Fn(A, B) -> O`).

It enables you to use a bunch of nifty functional programming paradigms that work well with iced.

## Required Methods

### `with`

```rust
fn with(self, prefix: A) -> impl Fn(B)
```

Applies the given first argument to a binary function and returns a new function that takes the other argument.

This lets you partially “apply” a function—equivalent to currying, but it only works with binary functions. If you want to apply an arbitrary number of arguments, create a little struct for them.

###### When is this useful?

Sometimes you will want to identify the source or target of some message in your user interface. This can be achieved through normal means by defining a closure and moving the identifier inside:

```rust
let id = 123;

element.map(move |result| Message::ButtonPressed(id, result))
```

That’s quite a mouthful. [`with`](trait.Function.md) lets you write:

```rust
let id = 123;

element.map(Message::ButtonPressed.with(id))
```

Effectively creating the same closure that partially applies the `id` to the message—but much more concise!

## Implementors

- `impl<F, A, B, O> Function<A, B, O> for F where F: Fn(A, B) -> O, A: Clone,`
