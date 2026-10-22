---
title: "Widget"
kind: trait
path: "iced_widget::Widget"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/trait.Widget.md"
docs_url: "https://docs.iced.rs/iced_widget/trait.Widget.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "180-354"
---

# `iced_widget::Widget`

```rust
pub trait Widget<Message, Theme = Theme, Renderer = Renderer>: Widget<Message, Theme, Renderer> {
    // Provided methods
    fn map<F, B>(self, f: F) -> Map<Self, F, Message>
       where Self: Sized,
             F: Fn(Message) -> B { ... }
    fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer>
       where Self: Sized + 'a { ... }
    fn explain(
        self,
        color: impl Into<Color>,
    ) -> impl Widget<Message, Theme, Renderer>
       where Self: Sized,
             Renderer: Renderer { ... }
}
```

A widget that can be rendered by iced.

This trait extends [`core::Widget`] with a few convenient methods—like [`Widget::map`](trait.Widget.md), [`Widget::boxed`](trait.Widget.md), and [`Widget::explain`](trait.Widget.md)—and defaults the `Theme` and `Renderer` parameters to iced’s.

**Source:** [lib.rs](../src/iced_widget/lib.rs.md) (lines 180-354)

## Provided Methods

### `map`

```rust
fn map<F, B>(self, f: F) -> Map<Self, F, Message> where Self: Sized, F: Fn(Message) -> B,
```

Applies a transformation to the produced message of this [`Widget`](trait.Widget.md).

This method is useful when you want to decouple different parts of your UI and make them **composable**.

###### Example

Imagine we want to use [our counter](index.md). But instead of showing a single counter, we want to display many of them. We can reuse the `Counter` type as it is!

We use composition to model the **state** of our new application:

```rust
use counter::Counter;

struct ManyCounters {
    counters: Vec<Counter>,
}
```

We can store the state of multiple counters now. However, the **messages** we implemented before describe the user interactions of a **single** counter. Right now, we need to also identify which counter is receiving user interactions. Can we use composition again? Yes.

```rust
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Counter(usize, counter::Message)
}
```

We compose the previous **messages** with the index of the counter producing them. Let’s implement our **view logic** now:

```rust
use counter::Counter;

use iced::widget::row;
use iced::{Function, Widget};

struct ManyCounters {
    counters: Vec<Counter>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Counter(usize, counter::Message),
}

impl ManyCounters {
    pub fn view(&self) -> impl Widget<Message, iced_widget::Theme, ()> {
        // We can quickly populate a `row` by mapping our counters
        row(
            self.counters
                .iter()
                .map(Counter::view)
                .enumerate()
                .map(|(index, counter)| {
                    // Here we turn our `Element<counter::Message>` into
                    // an `Element<Message>` by combining the `index` and the
                    // message of the `element`.
                    counter.map(Message::Counter.with(index)).boxed()
                }),
        )
    }
}
```

Finally, our **update logic** is pretty straightforward: simple delegation.

```rust
impl ManyCounters {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Counter(index, counter_msg) => {
                if let Some(counter) = self.counters.get_mut(index) {
                    counter.update(counter_msg);
                }
            }
        }
    }
}
```

### `boxed`

```rust
fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer> where Self: Sized + 'a,
```

Boxes this [`Widget`](trait.Widget.md), turning it into a generic [`Element`](type.Element.md).

### `explain`

```rust
fn explain( self, color: impl Into<Color>, ) -> impl Widget<Message, Theme, Renderer> where Self: Sized, Renderer: Renderer,
```

Marks this [`Widget`](trait.Widget.md) as *to-be-explained*.

The [`Renderer`](type.Renderer.md) will explain its layout graphically. This can be very useful for debugging your layout!

## Implementors

- `impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for T where T: Widget<Message, Theme, Renderer>,`
