---
title: "Program"
kind: trait
path: "iced::Program"
crate: "iced"
module: "iced"
file: "iced/trait.Program.md"
docs_url: "https://docs.iced.rs/iced/trait.Program.html"
---

# `iced::Program`

```rust
pub trait Program: Sized {
    type State;
    type Message: Send + 'static;
    type Theme: Base + 'static;
    type Renderer: Renderer + 'static;
    type Executor: Executor;

    // Required methods
    fn name() -> &'static str;
    fn settings(&self) -> Settings;
    fn window(&self) -> Option<Settings>;
    fn boot(&self) -> (Self::State, Task<Self::Message>);
    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Task<Self::Message>;
    fn view<'a>(
        &self,
        state: &'a Self::State,
        window: Id,
    ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a;

    // Provided methods
    fn title(&self, _state: &Self::State, _window: Id) -> String { ... }
    fn subscription(&self, _state: &Self::State) -> Subscription<Self::Message> { ... }
    fn theme(&self, _state: &Self::State, _window: Id) -> Option<Self::Theme> { ... }
    fn style(&self, _state: &Self::State, theme: &Self::Theme) -> Style { ... }
    fn scale_factor(&self, _state: &Self::State, _window: Id) -> f32 { ... }
    fn presets(&self) -> &[Preset<Self::State, Self::Message>] { ... }
}
```

An interactive, native, cross-platform, multi-windowed application.

A [`Program`](trait.Program.md) can execute asynchronous actions by returning a [`Task`](struct.Task.md) in some of its methods.

## Required Associated Types

### `State`

```rust
type State
```

The state of the program.

### `Message`

```rust
type Message: Send + 'static
```

The message of the program.

### `Theme`

```rust
type Theme: Base + 'static
```

The theme of the program.

### `Renderer`

```rust
type Renderer: Renderer + 'static
```

The renderer of the program.

### `Executor`

```rust
type Executor: Executor
```

The executor of the program.

## Required Methods

### `name`

```rust
fn name() -> &'static str
```

Returns the unique name of the [`Program`](trait.Program.md).

### `settings`

```rust
fn settings(&self) -> Settings
```

### `window`

```rust
fn window(&self) -> Option<Settings>
```

### `boot`

```rust
fn boot(&self) -> (Self::State, Task<Self::Message>)
```

### `update`

```rust
fn update( &self, state: &mut Self::State, message: Self::Message, ) -> Task<Self::Message>
```

### `view`

```rust
fn view<'a>( &self, state: &'a Self::State, window: Id, ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a
```

## Provided Methods

### `title`

```rust
fn title(&self, _state: &Self::State, _window: Id) -> String
```

### `subscription`

```rust
fn subscription(&self, _state: &Self::State) -> Subscription<Self::Message>
```

### `theme`

```rust
fn theme(&self, _state: &Self::State, _window: Id) -> Option<Self::Theme>
```

### `style`

```rust
fn style(&self, _state: &Self::State, theme: &Self::Theme) -> Style
```

### `scale_factor`

```rust
fn scale_factor(&self, _state: &Self::State, _window: Id) -> f32
```

### `presets`

```rust
fn presets(&self) -> &[Preset<Self::State, Self::Message>]
```

## Implementors

- `impl<P: Program> Program for Application<P>`
  - `type State = <P as Program>::State`
  - `type Message = <P as Program>::Message`
  - `type Theme = <P as Program>::Theme`
  - `type Renderer = <P as Program>::Renderer`
  - `type Executor = <P as Program>::Executor`
- `impl<P: Program> Program for Daemon<P>`
  - `type State = <P as Program>::State`
  - `type Message = <P as Program>::Message`
  - `type Theme = <P as Program>::Theme`
  - `type Renderer = <P as Program>::Renderer`
  - `type Executor = <P as Program>::Executor`
