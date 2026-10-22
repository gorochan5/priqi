---
title: "transition"
kind: function
path: "iced_widget::transition"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.transition.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.transition.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1769-1778"
---

# `iced_widget::transition`

```rust
pub fn transition<'a, Message, W, P>(
    value: P::Value,
    init: impl Fn() -> P + 'a,
    view: impl Fn(&P, Instant) -> W + 'a,
) -> Transition<'a, Message, W, P> where
    P: Program,
```

Creates a new [`Transition`](transition/struct.Transition.md).

The `init` closure will be used to initialize an implementor of [`Program`](transition/trait.Program.md). This is normally an `Animation`, but you can implement [`Program`](transition/trait.Program.md) on your own types as well.

The `view` closure will receive the [`Program`](transition/trait.Program.md) and the current [`Instant`](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html), which can be used for interpolating values. When the `value` changes, this will be called every frame, until the [`Program`](transition/trait.Program.md) stops animating.

## Example

Here is how you could implement a smooth progress bar:

```rust
use iced::widget::{transition, progress_bar};
use iced::Animation;

fn smooth_progress_bar<Message>(progress: f32) -> impl Widget<Message> {
    transition(progress, || Animation::new(0.).quick(), |animation, now| {
        progress_bar(0.0..=1.0, animation.interpolate_with(std::convert::identity, now))
    })
}
```

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1769-1778)
