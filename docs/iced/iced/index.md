---
title: "iced"
kind: crate
path: "iced"
crate: "iced"
file: "iced/index.md"
docs_url: "https://docs.iced.rs/iced/index.html"
source_file: "src/iced/lib.rs"
source_lines: "1-723"
---

# `iced`

iced is a cross-platform GUI library focused on simplicity and type-safety. Inspired by [Elm](https://elm-lang.org/).

## Disclaimer

iced is **experimental** software. If you expect the documentation to hold your hand as you learn the ropes, you are in for a frustrating experience.

The library leverages Rust to its full extent: ownership, borrowing, lifetimes, futures, streams, first-class functions, trait bounds, closures, and more. This documentation is not meant to teach you any of these. Far from it, it will assume you have **mastered** all of them.

Furthermore—just like Rust—iced is very unforgiving. It will not let you easily cut corners. The type signatures alone can be used to learn how to use most of the library. Everything is connected.

Therefore, iced is easy to learn for **advanced** Rust programmers; but plenty of patient beginners have learned it and had a good time with it. Since it leverages a lot of what Rust has to offer in a type-safe way, it can be a great way to discover Rust itself.

If you don’t like the sound of that, you expect to be spoonfed, or you feel frustrated and struggle to use the library; then I recommend you to wait patiently until [the book](https://book.iced.rs) is finished.

## The Pocket Guide

Start by calling [`run`](fn.run.md):

```rust
pub fn main() -> iced::Result {
    iced::run(update, view)
}
```

Define an `update` function to **change** your state:

```rust
fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}
```

Define a `view` function to **display** your state:

```rust
use iced::widget::{button, text};

fn view(counter: &u64) -> impl Widget<Message> {
    button(text(counter)).on_press(Message::Increment)
}
```

And create a `Message` enum to **connect** `view` and `update` together:

```rust
#[derive(Debug, Clone)]
enum Message {
    Increment,
}
```

### Custom State

You can define your own struct for your state:

```rust
#[derive(Default)]
struct Counter {
    value: u64,
}
```

But you have to change `update` and `view` accordingly:

```rust
fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> impl Widget<Message> {
    button(text(counter.value)).on_press(Message::Increment)
}
```

### Widgets and Elements

The `view` function must return a [`Widget`](trait.Widget.md). An [`Element`](type.Element.md) is a boxed [`Widget`](trait.Widget.md), which can be used just as well.

The [`widget`](widget/index.md) module contains a bunch of functions to help you build and use widgets.

Widgets are configured using the builder pattern:

```rust
use iced::widget::{button, column, text};

fn view(counter: &Counter) -> impl Widget<Message> {
    column![
        text(counter.value).size(20),
        button("Increment").on_press(Message::Increment),
    ]
    .spacing(10)
}
```

A widget can be turned into an [`Element`](type.Element.md) by calling `boxed`.

Widgets and elements are generic over the message type they produce. The [`Widget`](trait.Widget.md) returned by `view` must have the same `Message` type as your `update`.

### Layout

There is no unified layout system in iced. Instead, each widget implements its own layout strategy.

Building your layout will often consist in using a combination of [rows](widget/struct.Row.md), [columns](widget/struct.Column.md), and [containers](widget/struct.Container.md):

```rust
use iced::widget::{column, container, row};
use iced::Fill;

fn view(state: &State) -> impl Widget<Message> {
    container(
        column![
            "Top",
            row!["Left", "Right"].spacing(10),
            "Bottom"
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
}
```

Rows and columns lay out their children horizontally and vertically, respectively. [Spacing](widget/struct.Column.md) can be easily added between elements.

Containers position or align a single widget inside their bounds.

### Sizing

The width and height of widgets can generally be defined using a [`Length`](enum.Length.md).

- [`Fill`](enum.Length.md) will make the widget take all the available space in a given axis.
- [`Shrink`](enum.Length.md) will make the widget use its intrinsic size.

Most widgets use a [`Shrink`](enum.Length.md) sizing strategy by default, but will inherit a [`Fill`](enum.Length.md) strategy from their children.

A fixed numeric [`Length`](enum.Length.md) in [`Pixels`](struct.Pixels.md) can also be used:

```rust
use iced::widget::container;

fn view(state: &State) -> impl Widget<Message> {
    container("I am 300px tall!").height(300)
}
```

### Theming

The default [`Theme`](enum.Theme.md) of an application can be changed by defining a `theme` function and leveraging the [`Application`](application/struct.Application.md) builder, instead of directly calling [`run`](fn.run.md):

```rust
use iced::Theme;

pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .theme(theme)
        .run()
}

fn new() -> State {
    // ...
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}
```

The `theme` function takes the current state of the application, allowing the returned [`Theme`](enum.Theme.md) to be completely dynamic—just like `view`.

There are a bunch of built-in [`Theme`](enum.Theme.md) variants at your disposal, but you can also [create your own](enum.Theme.md).

### Styling

As with layout, iced does not have a unified styling system. However, all of the built-in widgets follow the same styling approach.

The appearance of a widget can be changed by calling its `style` method:

```rust
use iced::widget::container;

fn view(state: &State) -> impl Widget<Message> {
    container("I am a rounded box!").style(container::rounded_box)
}
```

The `style` method of a widget takes a closure that, given the current active [`Theme`](enum.Theme.md), returns the widget style:

```rust
use iced::widget::button;
use iced::Theme;

fn view(state: &State) -> impl Widget<Message> {
    button("I am a styled button!").style(|theme: &Theme, status| {
        let palette = theme.palette();

        match status {
            button::Status::Active => {
                button::Style::default()
                   .with_background(palette.success.strong.color)
            }
            _ => button::primary(theme, status),
        }
    })
}
```

Widgets that can be in multiple different states will also provide the closure with some [`Status`](widget/button/enum.Status.md), allowing you to use a different style for each state.

You can extract the [`Palette`](enum.Theme.md) colors of a [`Theme`](enum.Theme.md) with the [`palette`](enum.Theme.md) method.

Most widgets provide styling functions for your convenience in their respective modules; like [`container::rounded_box`](widget/container/fn.rounded_box.md), [`button::primary`](widget/button/fn.primary.md), or [`text::danger`](widget/text/fn.danger.md).

### Concurrent Tasks

The `update` function can *optionally* return a [`Task`](struct.Task.md).

A [`Task`](struct.Task.md) can be leveraged to perform asynchronous work, like running a future or a stream:

```rust
use iced::Task;

struct State {
    weather: Option<Weather>,
}

enum Message {
   FetchWeather,
   WeatherFetched(Weather),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchWeather => Task::perform(
            fetch_weather(),
            Message::WeatherFetched,
        ),
        Message::WeatherFetched(weather) => {
            state.weather = Some(weather);

            Task::none()
       }
    }
}

async fn fetch_weather() -> Weather {
    // ...
}
```

Tasks can also be used to interact with the iced runtime. Some modules expose functions that create tasks for different purposes—like [changing window settings](window/index.md), [focusing a widget](widget/operation/fn.focus_next.md), or querying the visible bounds of a widget with the `selector` module.

Like futures and streams, tasks expose [a monadic interface](struct.Task.md)—but they can also be [mapped](struct.Task.md), [chained](struct.Task.md), [batched](struct.Task.md), [canceled](struct.Task.md), and more.

### Passive Subscriptions

Applications can subscribe to passive sources of data—like time ticks or runtime events.

You will need to define a `subscription` function and use the [`Application`](application/struct.Application.md) builder:

```rust
use iced::window;
use iced::{Size, Subscription};

#[derive(Debug, Clone)]
enum Message {
    WindowResized(Size),
}

pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .subscription(subscription)
        .run()
}

fn subscription(state: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}
```

A [`Subscription`](struct.Subscription.md) is [a *declarative* builder of streams](struct.Subscription.md) that are not allowed to end on their own. Only the `subscription` function dictates the active subscriptions—just like `view` fully dictates the visible widgets of your user interface, at every moment.

As with tasks, some modules expose convenient functions that build a [`Subscription`](struct.Subscription.md) for you—like `time::every` which can be used to listen to time, or [`keyboard::listen`](keyboard/fn.listen.md) which will notify you of any keyboard events. But you can also create your own with [`Subscription::run`](struct.Subscription.md) and [`run_with`](struct.Subscription.md).

### Scaling Applications

The `update`, `view`, and `Message` triplet composes very nicely.

A common pattern is to leverage this composability to split an application into different screens:

```rust
use contacts::Contacts;
use conversation::Conversation;

use iced::{Task, Widget};

struct State {
    screen: Screen,
}

enum Screen {
    Contacts(Contacts),
    Conversation(Conversation),
}

enum Message {
   Contacts(contacts::Message),
   Conversation(conversation::Message)
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Contacts(message) => {
            if let Screen::Contacts(contacts) = &mut state.screen {
                let action = contacts.update(message);

                match action {
                    contacts::Action::None => Task::none(),
                    contacts::Action::Run(task) => task.map(Message::Contacts),
                    contacts::Action::Chat(contact) => {
                        let (conversation, task) = Conversation::new(contact);

                        state.screen = Screen::Conversation(conversation);

                        task.map(Message::Conversation)
                    }
                 }
            } else {
                Task::none()
            }
        }
        Message::Conversation(message) => {
            if let Screen::Conversation(conversation) = &mut state.screen {
                conversation.update(message).map(Message::Conversation)
            } else {
                Task::none()
            }
        }
    }
}

fn view(state: &State) -> impl Widget<Message> {
    match &state.screen {
        Screen::Contacts(contacts) => contacts.view().map(Message::Contacts).boxed(),
        Screen::Conversation(conversation) => conversation.view().map(Message::Conversation).boxed(),
    }
}
```

The `update` method of a screen can return an `Action` enum that can be leveraged by the parent to execute a task or transition to a completely different screen altogether. The variants of `Action` can have associated data. For instance, in the example above, the `Conversation` screen is created when `Contacts::update` returns an `Action::Chat` with the selected contact.

Effectively, this approach lets you “tell a story” to connect different screens together in a type safe way.

Furthermore, functor methods like [`Task::map`](struct.Task.md), [`Widget::map`](trait.Widget.md), and [`Subscription::map`](struct.Subscription.md) make composition seamless.

## Re-exports

- [`Application`](application/struct.Application.md)
- [`Daemon`](daemon/struct.Daemon.md)
- `pub use iced_futures::futures;`
- [`iced_highlighter`](../iced_highlighter/index.md) *(highlighter)*
- `pub use iced_renderer::wgpu::wgpu;wgpu-bare` *(wgpu-bare)*
- [`Center`](enum.Alignment.md)
- [`Fill`](enum.Length.md)
- [`FillPortion`](enum.Length.md)
- [`Fit`](enum.Length.md)
- [`Shrink`](enum.Length.md)
- [`Left`](alignment/enum.Horizontal.md)
- [`Right`](alignment/enum.Horizontal.md)
- [`Bottom`](alignment/enum.Vertical.md)
- [`Top`](alignment/enum.Vertical.md)

## Modules

- [`advanced`](advanced/index.md) *(advanced)* — Leverage advanced concepts like custom widgets.
- [`alignment`](alignment/index.md) — Align and position widgets.
- [`animation`](animation/index.md) — Animate your applications.
- [`application`](application/index.md) — Create and run iced applications step by step.
- [`backend`](backend/index.md) — Graphical backends are designed to aid in rendering computer graphics to a monitor.
- [`border`](border/index.md) — Draw lines around containers.
- [`clipboard`](clipboard/index.md) — Access the clipboard.
- [`color`](color/index.md) — Manage colors in different color spaces.
- [`daemon`](daemon/index.md) — Create and run daemons that run in the background.
- [`debug`](debug/index.md) — Debug your applications.
- [`event`](event/index.md) — Handle events of a user interface.
- [`executor`](executor/index.md) — Choose your preferred executor to power your application.
- [`font`](font/index.md) — Load and use fonts.
- [`gradient`](gradient/index.md) — Colors that transition progressively.
- [`keyboard`](keyboard/index.md) — Listen and react to keyboard events.
- [`message`](message/index.md) — Traits for the message type of a [`Program`](trait.Program.md).
- [`mouse`](mouse/index.md) — Listen and react to mouse events.
- [`overlay`](overlay/index.md) — Display interactive elements on top of other widgets.
- [`padding`](padding/index.md) — Space stuff around the perimeter.
- [`stream`](stream/index.md) — Create asynchronous streams of data.
- [`system`](system/index.md) — Retrieve system information.
- [`task`](task/index.md) — Create runtime tasks.
- [`theme`](theme/index.md) — Use the built-in theme and styles.
- [`time`](time/index.md) — Listen and react to time.
- [`touch`](touch/index.md) — Listen and react to touch events.
- [`widget`](widget/index.md) — Use the built-in widgets or create your own.
- [`window`](window/index.md) — Configure the window of your application in native platforms.

## Macros

- [`color`](macro.color.md) — Creates a [`Color`](struct.Color.md) with shorter and cleaner syntax.

## Structs

- [`Animation`](struct.Animation.md) — The animation of some particular state.
- [`Border`](struct.Border.md) — A border.
- [`Color`](struct.Color.md) — A color in the `sRGB` color space.
- [`Degrees`](struct.Degrees.md) — Degrees
- [`Font`](struct.Font.md) — A font.
- [`Padding`](struct.Padding.md) — An amount of space to pad for each side of a box
- [`Pixels`](struct.Pixels.md) — An amount of logical pixels.
- [`Point`](struct.Point.md) — A 2D point.
- [`Preset`](struct.Preset.md) — A specific boot strategy for a [`Program`](trait.Program.md).
- [`Radians`](struct.Radians.md) — Radians
- [`Rectangle`](struct.Rectangle.md) — An axis-aligned rectangle.
- [`Settings`](struct.Settings.md) — The settings of an iced program.
- [`Shadow`](struct.Shadow.md) — A shadow.
- [`Size`](struct.Size.md) — An amount of space in 2 dimensions.
- [`Subscription`](struct.Subscription.md) — A request to listen to external events.
- [`Task`](struct.Task.md) — A set of concurrent actions to be performed by the iced runtime.
- [`Transformation`](struct.Transformation.md) — A 2D transformation matrix.
- [`Vector`](struct.Vector.md) — A 2D vector.

## Enums

- [`Alignment`](enum.Alignment.md) — Alignment on the axis of a container.
- [`Backend`](enum.Backend.md) — A graphical backend.
- [`Background`](enum.Background.md) — The background of some element.
- [`Code`](enum.Code.md) — A specific region of code.
- [`ContentFit`](enum.ContentFit.md) — The strategy used to fit the contents of a widget to its bounding box.
- [`Error`](enum.Error.md) — An error that occurred while running an application.
- [`Event`](enum.Event.md) — A user interface event.
- [`Gradient`](enum.Gradient.md) — A fill which transitions colors progressively along a direction, either linearly, radially (TBD), or conically (TBD).
- [`Length`](enum.Length.md) — The strategy used to fill space in a specific dimension.
- [`Never`](enum.Never.md) — The error type for errors that can never happen.
- [`PowerPreference`](enum.PowerPreference.md) — The power-usage preference for graphics adapters.
- [`Rotation`](enum.Rotation.md) — The strategy used to rotate the content.
- [`Theme`](enum.Theme.md) — A built-in theme.

## Traits

- [`Executor`](trait.Executor.md) — A type that can run futures.
- [`Function`](trait.Function.md) — A trait extension for binary functions (`Fn(A, B) -> O`).
- [`Program`](trait.Program.md) — An interactive, native, cross-platform, multi-windowed application.
- [`Widget`](trait.Widget.md) — A widget that can be rendered by iced.
- [`Window`](trait.Window.md) — A window managed by iced.

## Functions

- [`application`](fn.application.md) — Creates an iced [`Application`](application/struct.Application.md) given its boot, update, and view logic.
- [`daemon`](fn.daemon.md) — Creates an iced [`Daemon`](daemon/struct.Daemon.md) given its boot, update, and view logic.
- [`exit`](fn.exit.md) — Creates a [`Task`](struct.Task.md) that exits the iced runtime.
- [`never`](fn.never.md) — A function that can *never* be called.
- [`run`](fn.run.md) — Runs a basic iced application with default [`Settings`](struct.Settings.md) given its update and view logic.

## Type Aliases

- [`Element`](type.Element.md) — A boxed [`Widget`](trait.Widget.md).
- [`Renderer`](type.Renderer.md) — The default graphics renderer for [`iced`](https://github.com/iced-rs/iced).
- [`Result`](type.Result.md) — The result of running an iced program.
