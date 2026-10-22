---
title: "widget"
kind: module
path: "iced::widget"
crate: "iced"
module: "iced"
file: "iced/widget/index.md"
docs_url: "https://docs.iced.rs/iced/widget/index.html"
source_file: "src/iced/lib.rs"
source_lines: "635"
---

# `iced::widget`

Use the built-in widgets or create your own.

## Modules

- [`button`](button/index.md) — Buttons allow your users to perform actions by pressing them.
- [`canvas`](canvas/index.md) *(canvas)* — Canvases can be leveraged to draw interactive 2D graphics.
- [`checkbox`](checkbox/index.md) — Checkboxes can be used to let users make binary choices.
- [`combo_box`](combo_box/index.md) — Combo boxes display a dropdown list of searchable and selectable options.
- [`component`](component/index.md) — Build and reuse custom widgets using The Elm Architecture.
- [`container`](container/index.md) — Containers let you align a widget inside their boundaries.
- [`float`](float/index.md) — Make elements float!
- [`grid`](grid/index.md) — Distribute content on a grid.
- [`image`](image/index.md) *(image-without-codecs)* — Images display raster graphics in different formats (PNG, JPG, etc.).
- [`keyed`](keyed/index.md) — Keyed widgets can provide hints to ensure continuity.
- [`lazy`](lazy/index.md) — A widget that only rebuilds its contents when necessary.
- [`markdown`](markdown/index.md) *(markdown)* — Markdown widgets can parse and display Markdown.
- [`operation`](operation/index.md) — Change internal widget state.
- [`overlay`](overlay/index.md) — Display interactive elements on top of other widgets.
- [`pane_grid`](pane_grid/index.md) — Pane grids let your users split regions of your application and organize layout dynamically.
- [`pick_list`](pick_list/index.md) — Pick lists display a dropdown list of selectable options.
- [`progress_bar`](progress_bar/index.md) — Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.
- [`qr_code`](qr_code/index.md) *(qr_code)* — QR codes display information in a type of two-dimensional matrix barcode.
- [`radio`](radio/index.md) — Radio buttons let users choose a single option from a bunch of options.
- [`row`](row/index.md) — Distribute content horizontally.
- [`rule`](rule/index.md) — Rules divide space horizontally or vertically.
- [`scrollable`](scrollable/index.md) — Scrollables let users navigate an endless amount of content with a scrollbar.
- [`selector`](selector/index.md) *(selector)* — Find and query widgets in your applications.
- [`sensor`](sensor/index.md) — Generate messages when content pops in and out of view.
- [`shader`](shader/index.md) *(wgpu)* — A custom shader widget for wgpu applications.
- [`slider`](slider/index.md) — Sliders let users set a value by moving an indicator.
- [`space`](space/index.md) — Add some explicit spacing between elements.
- [`sticky`](sticky/index.md) — Stick elements to the viewport!
- [`svg`](svg/index.md) *(svg)* — Svg widgets display vector graphics in your application.
- [`table`](table/index.md) — Display tables.
- [`text`](text/index.md) — Draw and interact with text.
- [`text_editor`](text_editor/index.md) — Text editors display a multi-line text input for text editing.
- [`text_input`](text_input/index.md) — Text inputs display fields that can be filled with text.
- [`theme`](theme/index.md) — Use the built-in theme and styles.
- [`toggler`](toggler/index.md) — Togglers let users make binary choices by toggling a switch.
- [`tooltip`](tooltip/index.md) — Tooltips display a hint of information over some element when hovered.
- [`transition`](transition/index.md) — A widget to make animated views.
- [`vertical_slider`](vertical_slider/index.md) — Sliders let users set a value by moving an indicator.

## Macros

- [`column`](macro.column.md) — Creates a [`Column`](struct.Column.md) with the given children.
- [`grid`](macro.grid.md) — Creates a [`Grid`](struct.Grid.md) with the given children.
- [`keyed_column`](macro.keyed_column.md) — Creates a keyed [`Column`](keyed/struct.Column.md) with the given children.
- [`rich_text`](macro.rich_text.md) — Creates some [`Rich`](text/struct.Rich.md) text with the given spans.
- [`row`](macro.row.md) — Creates a [`Row`](struct.Row.md) with the given children.
- [`stack`](macro.stack.md) — Creates a [`Stack`](struct.Stack.md) with the given children.
- [`text`](macro.text.md) — Creates a new [`Text`](../advanced/widget/struct.Text.md) widget with the provided content.

## Structs

- [`Action`](struct.Action.md) — A runtime action that can be performed by some widgets.
- [`Button`](struct.Button.md) — A generic widget that produces a message when pressed.
- [`Canvas`](struct.Canvas.md) — A widget capable of drawing 2D graphics.
- [`Checkbox`](struct.Checkbox.md) — A box that can be checked.
- [`Column`](struct.Column.md) — A container that distributes its contents vertically.
- [`ComboBox`](struct.ComboBox.md) — A widget for searching and selecting a single value from a list of options.
- [`Container`](struct.Container.md) — A widget that aligns its contents inside of its boundaries.
- [`Explain`](struct.Explain.md) — A [`Widget`](../trait.Widget.md) that explains the layout of its child.
- [`Float`](struct.Float.md) — A widget that can make its contents float over other widgets.
- [`Grid`](struct.Grid.md) — A container that distributes its contents on a responsive grid.
- [`Hover`](struct.Hover.md) — A widget that displays another widget on top of it.
- [`Id`](struct.Id.md) — The identifier of a generic widget.
- [`Image`](struct.Image.md) — A frame that displays an image while keeping aspect ratio.
- [`Lazy`](struct.Lazy.md) — A widget that only rebuilds its contents when necessary.
- [`MouseArea`](struct.MouseArea.md) — Emit messages on mouse events.
- [`PaneGrid`](struct.PaneGrid.md) — A collection of panes distributed using either vertical or horizontal splits to completely fill the space available.
- [`PickList`](struct.PickList.md) — A widget for selecting a single value from a list of options.
- [`Pin`](struct.Pin.md) — A widget that positions its contents at some fixed coordinates inside of its boundaries.
- [`ProgressBar`](struct.ProgressBar.md) — A bar that displays progress.
- [`QRCode`](struct.QRCode.md) — A type of matrix barcode consisting of squares arranged in a grid which can be read by an imaging device, such as a camera.
- [`Radio`](struct.Radio.md) — A circular button representing a choice.
- [`Responsive`](struct.Responsive.md) — A widget that is aware of its dimensions.
- [`Row`](struct.Row.md) — A container that distributes its contents horizontally.
- [`Rule`](struct.Rule.md) — Display a horizontal or vertical rule for dividing content.
- [`Scrollable`](struct.Scrollable.md) — A widget that can vertically display an infinite amount of content with a scrollbar.
- [`Sensor`](struct.Sensor.md) — A widget that can generate messages when its content pops in and out of view.
- [`Shader`](struct.Shader.md) — A widget which can render custom shaders with Iced’s `wgpu` backend.
- [`Slider`](struct.Slider.md) — An horizontal bar and a handle that selects a single value from a range of values.
- [`Space`](struct.Space.md) — An amount of empty space.
- [`Stack`](struct.Stack.md) — A container that displays children on top of each other.
- [`Sticky`](struct.Sticky.md) — A widget that keeps its contents in view.
- [`Svg`](struct.Svg.md) — A vector graphics image.
- [`TextEditor`](struct.TextEditor.md) — A multi-line text input.
- [`TextInput`](struct.TextInput.md) — A field that can be filled with text.
- [`Themer`](struct.Themer.md) — A widget that applies any `Theme` to its contents.
- [`Toggler`](struct.Toggler.md) — A toggler widget.
- [`Tooltip`](struct.Tooltip.md) — An element to display a widget over another.
- [`VerticalSlider`](struct.VerticalSlider.md) — An vertical bar and a handle that selects a single value from a range of values.
- [`Void`](struct.Void.md) — A zero-sized [`Widget`](../advanced/trait.Widget.md) that does nothing and will be filtered out by containers.

## Enums

- [`Theme`](enum.Theme.md) — A built-in theme.

## Traits

- [`Component`](trait.Component.md) — A reusable, custom widget that uses The Elm Architecture.
- [`Widget`](trait.Widget.md) — A widget that can be rendered by iced.

## Functions

- [`bottom`](fn.bottom.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space vertically and bottom-aligns its contents inside.
- [`bottom_center`](fn.bottom_center.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space and aligns its contents inside to the bottom center.
- [`bottom_right`](fn.bottom_right.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space and aligns its contents inside to the bottom right corner.
- [`button`](fn.button.md) — Creates a new [`Button`](struct.Button.md) with the provided content.
- [`canvas`](fn.canvas.md) *(canvas)* — Creates a new [`Canvas`](struct.Canvas.md).
- [`center`](fn.center.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space and centers its contents inside.
- [`center_x`](fn.center_x.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space horizontally and centers its contents inside.
- [`center_y`](fn.center_y.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space vertically and centers its contents inside.
- [`checkbox`](fn.checkbox.md) — Creates a new [`Checkbox`](struct.Checkbox.md).
- [`column`](fn.column.md) — Creates a new [`Column`](struct.Column.md) with the given children.
- [`combo_box`](fn.combo_box.md) — Creates a new [`ComboBox`](struct.ComboBox.md).
- [`component`](fn.component.md) — Turns an implementor of [`Component`](trait.Component.md) into an [`Element`](../advanced/widget/struct.Element.md) that can be embedded in any application.
- [`container`](fn.container.md) — Creates a new [`Container`](struct.Container.md) with the provided content.
- [`float`](fn.float.md) — Creates a new [`Float`](struct.Float.md) widget with the given content.
- [`grid`](fn.grid.md) — Creates a new [`Grid`](struct.Grid.md) from an iterator.
- [`hover`](fn.hover.md) — Displays a widget on top of another one, only when the base widget is hovered.
- [`iced`](fn.iced.md) — Creates an [`Element`](../advanced/widget/struct.Element.md) that displays the iced logo with the given `text_size`.
- [`image`](fn.image.md) *(image)* — Creates a new [`Image`](struct.Image.md).
- [`keyed_column`](fn.keyed_column.md) — Creates a new [`keyed::Column`](keyed/struct.Column.md) from an iterator of elements.
- [`lazy`](fn.lazy.md) — Creates a new [`Lazy`](struct.Lazy.md) widget with the given data `Dependency` and a closure that can turn this data into a widget tree.
- [`markdown`](fn.markdown.md) — Display a bunch of Markdown items.
- [`mouse_area`](fn.mouse_area.md) — Creates a new [`MouseArea`](struct.MouseArea.md).
- [`opaque`](fn.opaque.md) — Wraps the given widget and captures any mouse button presses inside the bounds of the widget—effectively making it *opaque*.
- [`pane_grid`](fn.pane_grid.md) — Creates a [`PaneGrid`](struct.PaneGrid.md) with the given [`pane_grid::State`](pane_grid/struct.State.md) and view function.
- [`pick_list`](fn.pick_list.md) — Creates a new [`PickList`](struct.PickList.md).
- [`pin`](fn.pin.md) — Creates a new [`Pin`](struct.Pin.md) widget with the given content.
- [`progress_bar`](fn.progress_bar.md) — Creates a new [`ProgressBar`](struct.ProgressBar.md).
- [`qr_code`](fn.qr_code.md) *(qr_code)* — Creates a new [`QRCode`](struct.QRCode.md) widget from the given [`Data`](qr_code/struct.Data.md).
- [`radio`](fn.radio.md) — Creates a new [`Radio`](struct.Radio.md).
- [`responsive`](fn.responsive.md) — Creates a new [`Responsive`](struct.Responsive.md) widget with a closure that produces its contents.
- [`rich_text`](fn.rich_text.md) — Creates a new [`Rich`](text/struct.Rich.md) text widget with the provided spans.
- [`right`](fn.right.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space horizontally and right-aligns its contents inside.
- [`right_center`](fn.right_center.md) — Creates a new [`Container`](struct.Container.md) that fills all the available space and aligns its contents inside to the right center.
- [`row`](fn.row.md) — Creates a new [`Row`](struct.Row.md) from an iterator.
- [`scrollable`](fn.scrollable.md) — Creates a new [`Scrollable`](struct.Scrollable.md) with the provided content.
- [`sensor`](fn.sensor.md) — Creates a new [`Sensor`](struct.Sensor.md) widget.
- [`shader`](fn.shader.md) *(wgpu)* — Creates a new [`Shader`](struct.Shader.md).
- [`slider`](fn.slider.md) — Creates a new [`Slider`](struct.Slider.md).
- [`space`](fn.space.md) — Creates some empty [`Space`](struct.Space.md) with no size.
- [`span`](fn.span.md) — Creates a new [`Span`](../advanced/text/struct.Span.md) of text with the provided content.
- [`stack`](fn.stack.md) — Creates a new [`Stack`](struct.Stack.md) with the given children.
- [`sticky`](fn.sticky.md) — Creates a new [`Sticky`](struct.Sticky.md) for the provided content.
- [`svg`](fn.svg.md) *(svg)* — Creates a new [`Svg`](struct.Svg.md) widget from the given [`Handle`](../advanced/svg/struct.Handle.md).
- [`table`](fn.table.md) — Creates a new [`Table`](table/struct.Table.md) with the given columns and rows.
- [`text`](fn.text.md) — Creates a new [`Text`](type.Text.md) widget with the provided content.
- [`text_editor`](fn.text_editor.md) — Creates a new [`TextEditor`](struct.TextEditor.md).
- [`text_input`](fn.text_input.md) — Creates a new [`TextInput`](struct.TextInput.md).
- [`themer`](fn.themer.md) — A widget that applies any `Theme` to its contents.
- [`toggler`](fn.toggler.md) — Creates a new [`Toggler`](struct.Toggler.md).
- [`tooltip`](fn.tooltip.md) — Creates a new [`Tooltip`](struct.Tooltip.md) for the provided content with the given [`Widget`](../advanced/trait.Widget.md) and [`tooltip::Position`](tooltip/enum.Position.md).
- [`transition`](fn.transition.md) — Creates a new [`Transition`](transition/struct.Transition.md).
- [`value`](fn.value.md) — Creates a new [`Text`](type.Text.md) widget that displays the provided value.
- [`vertical_slider`](fn.vertical_slider.md) — Creates a new [`VerticalSlider`](struct.VerticalSlider.md).
- [`void`](fn.void.md) — Creates a zero-sized [`Widget`](../advanced/trait.Widget.md) that does nothing and will be filtered out by containers.

## Type Aliases

- [`Element`](type.Element.md) — A boxed [`Widget`](../trait.Widget.md).
- [`Renderer`](type.Renderer.md) — The default graphics renderer for [`iced`](https://github.com/iced-rs/iced).
- [`Text`](type.Text.md) — A bunch of text.
