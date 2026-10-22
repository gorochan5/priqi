---
title: "iced_core"
kind: crate
path: "iced_core"
crate: "iced_core"
file: "iced_core/index.md"
docs_url: "https://docs.iced.rs/iced_core/index.html"
source_file: "src/iced_core/lib.rs"
source_lines: "1-157"
---

# `iced_core`

The core library of [Iced](https://github.com/iced-rs/iced).

This library holds basic types that can be reused and re-exported in different runtime implementations.

![The foundations of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/foundations.png?raw=true)

## Re-exports

- [`Alignment`](alignment/enum.Alignment.md)
- [`Animation`](animation/struct.Animation.md)
- [`Backend`](backend/enum.Backend.md)
- [`Border`](border/struct.Border.md)
- [`Clipboard`](clipboard/struct.Clipboard.md)
- [`Color`](color/struct.Color.md)
- [`Event`](event/enum.Event.md)
- [`Font`](font/struct.Font.md)
- [`Gradient`](gradient/enum.Gradient.md)
- [`Image`](image/struct.Image.md)
- [`InputMethod`](input_method/enum.InputMethod.md)
- [`Layout`](layout/struct.Layout.md)
- [`Length`](length/enum.Length.md)
- [`Overlay`](overlay/trait.Overlay.md)
- [`Padding`](padding/struct.Padding.md)
- [`Renderer`](renderer/trait.Renderer.md)
- [`Shell`](shell/struct.Shell.md)
- [`Svg`](svg/struct.Svg.md)
- [`Text`](text/struct.Text.md)
- [`Theme`](theme/enum.Theme.md)
- [`Element`](widget/struct.Element.md)
- [`Widget`](widget/trait.Widget.md)
- [`Window`](window/trait.Window.md)

## Modules

- [`alignment`](alignment/index.md) — Align and position widgets.
- [`animation`](animation/index.md) — Animate your applications.
- [`backend`](backend/index.md) — Graphical backends are designed to aid in rendering computer graphics to a monitor.
- [`border`](border/index.md) — Draw lines around containers.
- [`clipboard`](clipboard/index.md) — Access the clipboard.
- [`color`](color/index.md) — Manage colors in different color spaces.
- [`event`](event/index.md) — Handle events of a user interface.
- [`font`](font/index.md) — Load and use fonts.
- [`gradient`](gradient/index.md) — Colors that transition progressively.
- [`image`](image/index.md) — Load and draw raster graphics.
- [`input_method`](input_method/index.md) — Listen to input method events.
- [`keyboard`](keyboard/index.md) — Listen to keyboard events.
- [`layout`](layout/index.md) — Position your widgets properly.
- [`length`](length/index.md) — Describe amounts of space accurately.
- [`mouse`](mouse/index.md) — Handle mouse events.
- [`overlay`](overlay/index.md) — Display interactive elements on top of other widgets.
- [`padding`](padding/index.md) — Space stuff around the perimeter.
- [`renderer`](renderer/index.md) — Write your own renderer.
- [`shell`](shell/index.md) — Communicate with the iced runtime from widgets.
- [`svg`](svg/index.md) — Load and draw vector graphics.
- [`text`](text/index.md) — Draw and interact with text.
- [`theme`](theme/index.md) — Use the built-in theme and styles.
- [`time`](time/index.md) — Keep track of time, both in native and web platforms!
- [`touch`](touch/index.md) — Build touch events.
- [`widget`](widget/index.md) — Create custom widgets and operate on them.
- [`window`](window/index.md) — Build window-based GUI applications.

## Macros

- [`color`](macro.color.md) — Creates a [`Color`](color/struct.Color.md) with shorter and cleaner syntax.

## Structs

- [`Bytes`](struct.Bytes.md) — A cheaply cloneable and sliceable chunk of contiguous memory.
- [`Degrees`](struct.Degrees.md) — Degrees
- [`Pixels`](struct.Pixels.md) — An amount of logical pixels.
- [`Point`](struct.Point.md) — A 2D point.
- [`Radians`](struct.Radians.md) — Radians
- [`Rectangle`](struct.Rectangle.md) — An axis-aligned rectangle.
- [`Settings`](struct.Settings.md) — The settings of an iced program.
- [`Shadow`](struct.Shadow.md) — A shadow.
- [`Size`](struct.Size.md) — An amount of space in 2 dimensions.
- [`SmolStr`](struct.SmolStr.md) — A `SmolStr` is a string type that has the following properties:
- [`Transformation`](struct.Transformation.md) — A 2D transformation matrix.
- [`Vector`](struct.Vector.md) — A 2D vector.

## Enums

- [`Background`](enum.Background.md) — The background of some element.
- [`Code`](enum.Code.md) — A specific region of code.
- [`ContentFit`](enum.ContentFit.md) — The strategy used to fit the contents of a widget to its bounding box.
- [`Never`](enum.Never.md) — The error type for errors that can never happen.
- [`Rotation`](enum.Rotation.md) — The strategy used to rotate the content.

## Traits

- [`Function`](trait.Function.md) — A trait extension for binary functions (`Fn(A, B) -> O`).

## Functions

- [`never`](fn.never.md) — A function that can *never* be called.
