---
title: "Application"
kind: struct
path: "iced::application::Application"
crate: "iced"
module: "iced::application"
file: "iced/application/struct.Application.md"
docs_url: "https://docs.iced.rs/iced/application/struct.Application.html"
source_file: "src/iced/application.rs"
source_lines: "176-181"
---

# `iced::application::Application`

```rust
pub struct Application<P: Program> { /* private fields */ }
```

The underlying definition and configuration of an iced application.

You can use this API to create and run iced applications step by step—without coupling your logic to a trait or a specific type.

You can create an [`Application`](struct.Application.md) with the [`application`](../fn.application.md) helper.

**Source:** [application.rs](../../src/iced/application.rs.md) (lines 176-181)

## Methods

### `impl<P: Program> Application<P>`

#### `run`

```rust
pub fn run(self) -> Result where Self: 'static, P::Message: MaybeDebug + MaybeClone,
```

Runs the [`Application`](struct.Application.md).

#### `settings`

```rust
pub fn settings(self, settings: Settings) -> Self
```

Sets the [`Settings`](../struct.Settings.md) that will be used to run the [`Application`](struct.Application.md).

#### `antialiasing`

```rust
pub fn antialiasing(self, antialiasing: bool) -> Self
```

Sets the [`Settings::antialiasing`](../struct.Settings.md) of the [`Application`](struct.Application.md).

#### `font`

```rust
pub fn font(self, font: Font) -> Self
```

Sets the default [`Font`](../struct.Font.md) of the [`Application`](struct.Application.md).

#### `line_height`

```rust
pub fn line_height(self, line_height: LineHeight) -> Self
```

Sets the default [`text::LineHeight`](../advanced/text/enum.LineHeight.md) of the [`Application`](struct.Application.md).

#### `fonts`

```rust
pub fn fonts( self, fonts: impl IntoIterator<Item = impl Into<Cow<'static, [u8]>>>, ) -> Self
```

Adds a set of fonts to the list of fonts that will be loaded at the start of the [`Application`](struct.Application.md).

#### `window`

```rust
pub fn window(self, window: Settings) -> Self
```

Sets the [`window::Settings`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

Overwrites any previous [`window::Settings`](../window/struct.Settings.md).

#### `centered`

```rust
pub fn centered(self) -> Self
```

Sets the [`window::Settings::position`](../window/struct.Settings.md) to [`window::Position::Centered`](../window/enum.Position.md) in the [`Application`](struct.Application.md).

#### `exit_on_close_request`

```rust
pub fn exit_on_close_request(self, exit_on_close_request: bool) -> Self
```

Sets the [`window::Settings::exit_on_close_request`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `window_size`

```rust
pub fn window_size(self, size: impl Into<Size>) -> Self
```

Sets the [`window::Settings::size`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `transparent`

```rust
pub fn transparent(self, transparent: bool) -> Self
```

Sets the [`window::Settings::transparent`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `resizable`

```rust
pub fn resizable(self, resizable: bool) -> Self
```

Sets the [`window::Settings::resizable`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `decorations`

```rust
pub fn decorations(self, decorations: bool) -> Self
```

Sets the [`window::Settings::decorations`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `position`

```rust
pub fn position(self, position: Position) -> Self
```

Sets the [`window::Settings::position`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `level`

```rust
pub fn level(self, level: Level) -> Self
```

Sets the [`window::Settings::level`](../window/struct.Settings.md) of the [`Application`](struct.Application.md).

#### `backend`

```rust
pub fn backend(self, backend: Backend) -> Self
```

Sets the [`Backend`](../enum.Backend.md) of the [`Application`](struct.Application.md).

#### `power_preference`

```rust
pub fn power_preference(self, power_preference: PowerPreference) -> Self
```

Sets the [`backend::PowerPreference`](../enum.PowerPreference.md) of the [`Application`](struct.Application.md).

#### `title`

```rust
pub fn title( self, title: impl TitleFn<P::State>, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
```

Sets the title of the [`Application`](struct.Application.md).

#### `subscription`

```rust
pub fn subscription( self, f: impl Fn(&P::State) -> Subscription<P::Message>, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
```

Sets the subscription logic of the [`Application`](struct.Application.md).

#### `theme`

```rust
pub fn theme( self, f: impl ThemeFn<P::State, P::Theme>, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
```

Sets the theme logic of the [`Application`](struct.Application.md).

#### `style`

```rust
pub fn style( self, f: impl Fn(&P::State, &P::Theme) -> Style, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
```

Sets the style logic of the [`Application`](struct.Application.md).

#### `scale_factor`

```rust
pub fn scale_factor( self, f: impl Fn(&P::State) -> f32, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
```

Sets the scale factor of the [`Application`](struct.Application.md).

#### `executor`

```rust
pub fn executor<E>( self, ) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>> where E: Executor,
```

Sets the executor of the [`Application`](struct.Application.md).

#### `presets`

```rust
pub fn presets( self, presets: impl IntoIterator<Item = Preset<P::State, P::Message>>, ) -> Self
```

Sets the boot presets of the [`Application`](struct.Application.md).

Presets can be used to override the default booting strategy of your application during testing to create reproducible environments.

## Trait Implementations

### `impl<P: Debug + Program> Debug for Application<P> where P::State: Debug, P::Message: Debug,`

#### `fmt`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

### `impl<P: Program> Program for Application<P>`

#### `State`

```rust
type State = <P as Program>::State
```

The state of the program.

#### `Message`

```rust
type Message = <P as Program>::Message
```

The message of the program.

#### `Theme`

```rust
type Theme = <P as Program>::Theme
```

The theme of the program.

#### `Renderer`

```rust
type Renderer = <P as Program>::Renderer
```

The renderer of the program.

#### `Executor`

```rust
type Executor = <P as Program>::Executor
```

The executor of the program.

#### `name`

```rust
fn name() -> &'static str
```

Returns the unique name of the [`Program`](../trait.Program.md).

#### `settings`

```rust
fn settings(&self) -> Settings
```

#### `window`

```rust
fn window(&self) -> Option<Settings>
```

#### `boot`

```rust
fn boot(&self) -> (Self::State, Task<Self::Message>)
```

#### `update`

```rust
fn update( &self, state: &mut Self::State, message: Self::Message, ) -> Task<Self::Message>
```

#### `view`

```rust
fn view<'a>( &self, state: &'a Self::State, window: Id, ) -> impl Widget<P::Message, P::Theme, P::Renderer> + 'a
```

#### `title`

```rust
fn title(&self, state: &Self::State, window: Id) -> String
```

#### `subscription`

```rust
fn subscription(&self, state: &Self::State) -> Subscription<Self::Message>
```

#### `theme`

```rust
fn theme(&self, state: &Self::State, window: Id) -> Option<Self::Theme>
```

#### `style`

```rust
fn style(&self, state: &Self::State, theme: &Self::Theme) -> Style
```

#### `scale_factor`

```rust
fn scale_factor(&self, state: &Self::State, window: Id) -> f32
```

#### `presets`

```rust
fn presets(&self) -> &[Preset<Self::State, Self::Message>]
```

## Auto Trait Implementations

- `impl<P> Freeze for Application<P> where P: Freeze,`
- `impl<P> !RefUnwindSafe for Application<P>`
- `impl<P> !Send for Application<P>`
- `impl<P> !Sync for Application<P>`
- `impl<P> Unpin for Application<P> where P: Unpin,`
- `impl<P> UnsafeUnpin for Application<P> where P: UnsafeUnpin,`
- `impl<P> !UnwindSafe for Application<P>`
## Blanket Implementations

### `impl<T> Any for T where T: 'static + ?Sized,`

#### `type_id`

```rust
fn type_id(&self) -> TypeId
```

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

### `impl<T> Borrow<T> for T where T: ?Sized,`

#### `borrow`

```rust
fn borrow(&self) -> &T
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<T> BorrowMut<T> for T where T: ?Sized,`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut T
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<State, Message> IntoBoot<State, Message> for State`

#### `into_boot`

```rust
fn into_boot(self) -> (State, Task<Message>)
```

Turns some type into the initial state of some [`Application`](struct.Application.md).

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<T> Same for T`

#### `Output`

```rust
type Output = T
```

Should always be `Self`

### `impl<T, U> TryFrom<U> for T where U: Into<T>,`

#### `Error`

```rust
type Error = Infallible
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
```

Performs the conversion.

### `impl<T, U> TryInto<U> for T where U: TryFrom<T>,`

#### `Error`

```rust
type Error = <U as TryFrom<T>>::Error
```

The type returned in the event of a conversion error.

#### `try_into`

```rust
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
```

Performs the conversion.

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<V, T> VZip<V> for T where V: MultiLane<T>,`

#### `vzip`

```rust
fn vzip(self) -> V
```

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeDebug for T where T: Debug,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`
