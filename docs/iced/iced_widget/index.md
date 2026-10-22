---
title: "iced_widget"
kind: crate
path: "iced_widget"
crate: "iced_widget"
file: "iced_widget/index.md"
docs_url: "https://docs.iced.rs/iced_widget/index.html"
source_file: "src/iced_widget/lib.rs"
source_lines: "1-493"
---

# `iced_widget`

Use the built-in widgets or create your own.

## Re-exports

- [`Button`](button/struct.Button.md)
- [`Checkbox`](checkbox/struct.Checkbox.md)
- `pub use column::Column;`
- [`ComboBox`](combo_box/struct.ComboBox.md)
- [`Component`](component/trait.Component.md)
- [`Container`](container/struct.Container.md)
- [`Float`](float/struct.Float.md)
- [`Grid`](grid/struct.Grid.md)
- `pub use hover::Hover;`
- [`Lazy`](lazy/struct.Lazy.md)
- `pub use mouse_area::MouseArea;`
- [`PaneGrid`](pane_grid/struct.PaneGrid.md)
- [`PickList`](pick_list/struct.PickList.md)
- `pub use pin::Pin;`
- [`ProgressBar`](progress_bar/struct.ProgressBar.md)
- [`Radio`](radio/struct.Radio.md)
- `pub use responsive::Responsive;`
- [`Row`](row/struct.Row.md)
- [`Rule`](rule/struct.Rule.md)
- [`Scrollable`](scrollable/struct.Scrollable.md)
- [`Sensor`](sensor/struct.Sensor.md)
- [`Slider`](slider/struct.Slider.md)
- [`Space`](space/struct.Space.md)
- `pub use stack::Stack;`
- [`Sticky`](sticky/struct.Sticky.md)
- [`Text`](text/type.Text.md)
- [`TextEditor`](text_editor/struct.TextEditor.md)
- [`TextInput`](text_input/struct.TextInput.md)
- `pub use themer::Themer;`
- [`Toggler`](toggler/struct.Toggler.md)
- [`Tooltip`](tooltip/struct.Tooltip.md)
- [`VerticalSlider`](vertical_slider/struct.VerticalSlider.md)
- [`Shader`](shader/struct.Shader.md) *(wgpu)*
- [`Svg`](svg/struct.Svg.md) *(svg)*
- [`Image`](image/struct.Image.md) *(image)*
- [`Canvas`](canvas/struct.Canvas.md) *(canvas)*
- [`QRCode`](qr_code/struct.QRCode.md) *(qr_code)*
- [`component`](component/fn.component.md)
- [`table`](table/fn.table.md)
- `pub use iced_renderer as renderer;`
- `pub use iced_renderer::core;`
- `pub use iced_renderer::graphics;`

## Modules

- [`button`](button/index.md) — Buttons allow your users to perform actions by pressing them.
- [`canvas`](canvas/index.md) *(canvas)* — Canvases can be leveraged to draw interactive 2D graphics.
- [`checkbox`](checkbox/index.md) — Checkboxes can be used to let users make binary choices.
- [`combo_box`](combo_box/index.md) — Combo boxes display a dropdown list of searchable and selectable options.
- [`component`](component/index.md) — Build and reuse custom widgets using The Elm Architecture.
- [`container`](container/index.md) — Containers let you align a widget inside their boundaries.
- [`float`](float/index.md) — Make elements float!
- [`grid`](grid/index.md) — Distribute content on a grid.
- [`image`](image/index.md) *(image)* — Images display raster graphics in different formats (PNG, JPG, etc.).
- [`keyed`](keyed/index.md) — Keyed widgets can provide hints to ensure continuity.
- [`lazy`](lazy/index.md) — A widget that only rebuilds its contents when necessary.
- [`markdown`](markdown/index.md) *(markdown)* — Markdown widgets can parse and display Markdown.
- [`overlay`](overlay/index.md) — Display interactive elements on top of other widgets.
- [`pane_grid`](pane_grid/index.md) — Pane grids let your users split regions of your application and organize layout dynamically.
- [`pick_list`](pick_list/index.md) — Pick lists display a dropdown list of selectable options.
- [`progress_bar`](progress_bar/index.md) — Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.
- [`qr_code`](qr_code/index.md) *(qr_code)* — QR codes display information in a type of two-dimensional matrix barcode.
- [`radio`](radio/index.md) — Radio buttons let users choose a single option from a bunch of options.
- [`row`](row/index.md) — Distribute content horizontally.
- [`rule`](rule/index.md) — Rules divide space horizontally or vertically.
- [`scrollable`](scrollable/index.md) — Scrollables let users navigate an endless amount of content with a scrollbar.
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

- [`column`](macro.column.md) — Creates a [`Column`] with the given children.
- [`grid`](macro.grid.md) — Creates a [`Grid`](grid/struct.Grid.md) with the given children.
- [`keyed_column`](macro.keyed_column.md) — Creates a keyed [`Column`](keyed/column/struct.Column.md) with the given children.
- [`rich_text`](macro.rich_text.md) — Creates some [`Rich`](text/struct.Rich.md) text with the given spans.
- [`row`](macro.row.md) — Creates a [`Row`](row/struct.Row.md) with the given children.
- [`stack`](macro.stack.md) — Creates a `Stack` with the given children.
- [`text`](macro.text.md) — Creates a new `Text` widget with the provided content.

## Structs

- [`Action`](struct.Action.md) — A runtime action that can be performed by some widgets.
- [`Explain`](struct.Explain.md) — A [`Widget`](trait.Widget.md) that explains the layout of its child.
- [`Id`](struct.Id.md) — The identifier of a generic widget.
- [`Void`](struct.Void.md) — A zero-sized [`Widget`] that does nothing and will be filtered out by containers.

## Enums

- [`Theme`](enum.Theme.md) — A built-in theme.

## Traits

- [`Widget`](trait.Widget.md) — A widget that can be rendered by iced.

## Functions

- [`bottom`](fn.bottom.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space vertically and bottom-aligns its contents inside.
- [`bottom_center`](fn.bottom_center.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the bottom center.
- [`bottom_right`](fn.bottom_right.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the bottom right corner.
- [`button`](fn.button.md) — Creates a new [`Button`](button/struct.Button.md) with the provided content.
- [`canvas`](fn.canvas.md) *(canvas)* — Creates a new [`Canvas`](canvas/struct.Canvas.md).
- [`center`](fn.center.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space and centers its contents inside.
- [`center_x`](fn.center_x.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space horizontally and centers its contents inside.
- [`center_y`](fn.center_y.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space vertically and centers its contents inside.
- [`checkbox`](fn.checkbox.md) — Creates a new [`Checkbox`](checkbox/struct.Checkbox.md).
- [`column`](fn.column.md) — Creates a new [`Column`] with the given children.
- [`combo_box`](fn.combo_box.md) — Creates a new [`ComboBox`](combo_box/struct.ComboBox.md).
- [`container`](fn.container.md) — Creates a new [`Container`](container/struct.Container.md) with the provided content.
- [`float`](fn.float.md) — Creates a new [`Float`](float/struct.Float.md) widget with the given content.
- [`grid`](fn.grid.md) — Creates a new [`Grid`](grid/struct.Grid.md) from an iterator.
- [`hover`](fn.hover.md) — Displays a widget on top of another one, only when the base widget is hovered.
- [`iced`](fn.iced.md) — Creates an [`Element`] that displays the iced logo with the given `text_size`.
- [`image`](fn.image.md) *(image)* — Creates a new [`Image`](image/struct.Image.md).
- [`keyed_column`](fn.keyed_column.md) — Creates a new [`keyed::Column`](keyed/column/struct.Column.md) from an iterator of elements.
- [`lazy`](fn.lazy.md) — Creates a new [`Lazy`](lazy/struct.Lazy.md) widget with the given data `Dependency` and a closure that can turn this data into a widget tree.
- [`markdown`](fn.markdown.md) *(markdown)* — Display a bunch of Markdown items.
- [`mouse_area`](fn.mouse_area.md) — Creates a new [`MouseArea`].
- [`opaque`](fn.opaque.md) — Wraps the given widget and captures any mouse button presses inside the bounds of the widget—effectively making it *opaque*.
- [`pane_grid`](fn.pane_grid.md) — Creates a [`PaneGrid`](pane_grid/struct.PaneGrid.md) with the given [`pane_grid::State`](pane_grid/state/struct.State.md) and view function.
- [`pick_list`](fn.pick_list.md) — Creates a new [`PickList`](pick_list/struct.PickList.md).
- [`pin`](fn.pin.md) — Creates a new [`Pin`] widget with the given content.
- [`progress_bar`](fn.progress_bar.md) — Creates a new [`ProgressBar`](progress_bar/struct.ProgressBar.md).
- [`qr_code`](fn.qr_code.md) *(qr_code)* — Creates a new [`QRCode`](qr_code/struct.QRCode.md) widget from the given [`Data`](qr_code/struct.Data.md).
- [`radio`](fn.radio.md) — Creates a new [`Radio`](radio/struct.Radio.md).
- [`responsive`](fn.responsive.md) — Creates a new [`Responsive`] widget with a closure that produces its contents.
- [`rich_text`](fn.rich_text.md) — Creates a new [`Rich`](text/struct.Rich.md) text widget with the provided spans.
- [`right`](fn.right.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space horizontally and right-aligns its contents inside.
- [`right_center`](fn.right_center.md) — Creates a new [`Container`](container/struct.Container.md) that fills all the available space and aligns its contents inside to the right center.
- [`row`](fn.row.md) — Creates a new [`Row`](row/struct.Row.md) from an iterator.
- [`scrollable`](fn.scrollable.md) — Creates a new [`Scrollable`](scrollable/struct.Scrollable.md) with the provided content.
- [`sensor`](fn.sensor.md) — Creates a new [`Sensor`](sensor/struct.Sensor.md) widget.
- [`shader`](fn.shader.md) *(wgpu)* — Creates a new [`Shader`](shader/struct.Shader.md).
- [`slider`](fn.slider.md) — Creates a new [`Slider`](slider/struct.Slider.md).
- [`space`](fn.space.md) — Creates some empty [`Space`](space/struct.Space.md) with no size.
- [`span`](fn.span.md) — Creates a new [`Span`](text/struct.Span.md) of text with the provided content.
- [`stack`](fn.stack.md) — Creates a new `Stack` with the given children.
- [`sticky`](fn.sticky.md) — Creates a new [`Sticky`](sticky/struct.Sticky.md) for the provided content.
- [`svg`](fn.svg.md) *(svg)* — Creates a new [`Svg`](svg/struct.Svg.md) widget from the given [`Handle`](svg/struct.Handle.md).
- [`text`](fn.text.md) — Creates a new [`Text`](text/type.Text.md) widget with the provided content.
- [`text_editor`](fn.text_editor.md) — Creates a new [`TextEditor`](text_editor/struct.TextEditor.md).
- [`text_input`](fn.text_input.md) — Creates a new [`TextInput`](text_input/struct.TextInput.md).
- [`themer`](fn.themer.md) — A widget that applies any `Theme` to its contents.
- [`toggler`](fn.toggler.md) — Creates a new [`Toggler`](toggler/struct.Toggler.md).
- [`tooltip`](fn.tooltip.md) — Creates a new [`Tooltip`](tooltip/struct.Tooltip.md) for the provided content with the given [`Widget`] and [`tooltip::Position`](tooltip/enum.Position.md).
- [`transition`](fn.transition.md) — Creates a new [`Transition`](transition/struct.Transition.md).
- [`value`](fn.value.md) — Creates a new [`Text`](text/type.Text.md) widget that displays the provided value.
- [`vertical_slider`](fn.vertical_slider.md) — Creates a new [`VerticalSlider`](vertical_slider/struct.VerticalSlider.md).
- [`void`](fn.void.md) — Creates a zero-sized [`Widget`] that does nothing and will be filtered out by containers.

## Type Aliases

- [`Element`](type.Element.md) — A boxed [`Widget`](trait.Widget.md).
- [`Renderer`](type.Renderer.md) — The default graphics renderer for [`iced`](https://github.com/iced-rs/iced).
