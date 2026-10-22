---
title: "iced_test"
kind: crate
path: "iced_test"
crate: "iced_test"
file: "iced_test/index.md"
docs_url: "https://docs.iced.rs/iced_test/index.html"
source_file: "src/iced_test/lib.rs"
source_lines: "1-297"
---

# `iced_test`

Test your `iced` applications in headless mode.

## Basic Usage

Let’s assume we want to test [the classical counter interface](https://book.iced.rs/architecture.html#dissecting-an-interface).

First, we will want to create a [`Simulator`](simulator/struct.Simulator.md) of our interface:

```rust
use iced_test::simulator;

let mut counter = Counter { value: 0 };
let mut ui = simulator(counter.view());
```

Now we can simulate a user interacting with our interface. Let’s use [`Simulator::click`](simulator/struct.Simulator.md) to click the counter buttons:

```rust
let _ = ui.click("+");
let _ = ui.click("+");
let _ = ui.click("-");
```

[`Simulator::click`](simulator/struct.Simulator.md) takes a type implementing the [`Selector`](trait.Selector.md) trait. A [`Selector`](trait.Selector.md) describes a way to query the widgets of an interface. In this case, we leverage the [`Selector`](trait.Selector.md) implementation of `&str`, which selects a widget by the text it contains.

We can now process any messages produced by these interactions and then assert that the final value of our counter is indeed `1`!

```rust
for message in ui.into_messages() {
    counter.update(message);
}

assert_eq!(counter.value, 1);
```

We can even rebuild the interface to make sure the counter *displays* the proper value with [`Simulator::find`](simulator/struct.Simulator.md):

```rust
let mut ui = simulator(counter.view());

assert!(ui.find("1").is_ok(), "Counter should display 1!");
```

And that’s it! That’s the gist of testing `iced` applications!

[`Simulator`](simulator/struct.Simulator.md) contains additional operations you can use to simulate more interactions—like [`tap_key`](simulator/struct.Simulator.md) or [`typewrite`](simulator/struct.Simulator.md)—and even perform [*snapshot testing*](simulator/struct.Simulator.md)!

## Re-exports

- [`Emulator`](emulator/struct.Emulator.md)
- [`Ice`](ice/struct.Ice.md)
- [`Instruction`](instruction/enum.Instruction.md)
- [`Simulator`](simulator/struct.Simulator.md)
- [`simulator`](simulator/fn.simulator.md)
- `pub use iced_futures as futures;`
- `pub use iced_program as program;`
- `pub use iced_renderer as renderer;`
- `pub use iced_runtime as runtime;`
- `pub use iced_runtime::core;`
- `pub use iced_selector as selector;`

## Modules

- [`emulator`](emulator/index.md) — Run your application in a headless runtime.
- [`ice`](ice/index.md) — A shareable, simple format of end-to-end tests.
- [`instruction`](instruction/index.md) — A step in an end-to-end test.
- [`simulator`](simulator/index.md) — Run a simulation of your application without side effects.

## Enums

- [`Error`](enum.Error.md) — A test error.

## Traits

- [`Selector`](trait.Selector.md) — A type that traverses the widget tree to “select” data and produce some output.

## Functions

- [`run`](fn.run.md) — Runs an [`Ice`](ice/struct.Ice.md) test suite for the given `Program`.
- [`screenshot`](fn.screenshot.md) — Takes a screenshot of the given `Program` with the given theme, viewport, and scale factor after running it for the given [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html).
