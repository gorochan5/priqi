---
title: "window"
kind: module
path: "iced_runtime::window"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/window/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/index.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "1-479"
---

# `iced_runtime::window`

Build window-based GUI applications.

## Enums

- [`Action`](enum.Action.md) — An operation to be performed on some window.

## Functions

- [`allow_automatic_tabbing`](fn.allow_automatic_tabbing.md) — Sets whether the system can automatically organize windows into tabs.
- [`close`](fn.close.md) — Closes the window with `id`.
- [`close_events`](fn.close_events.md) — Subscribes to all [`Event::Closed`] occurrences in the running application.
- [`close_requests`](fn.close_requests.md) — Subscribes to all [`Event::CloseRequested`] occurrences in the running application.
- [`disable_mouse_passthrough`](fn.disable_mouse_passthrough.md) — Disables mouse passthrough for the given window.
- [`drag`](fn.drag.md) — Begins dragging the window while the left mouse button is held.
- [`drag_resize`](fn.drag_resize.md) — Begins resizing the window while the left mouse button is held.
- [`enable_mouse_passthrough`](fn.enable_mouse_passthrough.md) — Enables mouse passthrough for the given window.
- [`events`](fn.events.md) — Subscribes to all window events of the running application.
- [`frames`](fn.frames.md) — Subscribes to the frames of the window of the running application.
- [`gain_focus`](fn.gain_focus.md) — Brings the window to the front and sets input focus. Has no effect if the window is already in focus, minimized, or not visible.
- [`is_maximized`](fn.is_maximized.md) — Gets the maximized state of the window with the given [`Id`].
- [`is_minimized`](fn.is_minimized.md) — Gets the minimized state of the window with the given [`Id`].
- [`latest`](fn.latest.md) — Gets the window [`Id`] of the latest window.
- [`maximize`](fn.maximize.md) — Maximizes the window.
- [`minimize`](fn.minimize.md) — Minimizes the window.
- [`mode`](fn.mode.md) — Gets the current [`Mode`] of the window.
- [`monitor_size`](fn.monitor_size.md) — Gets the logical dimensions of the monitor containing the window with the given [`Id`].
- [`move_to`](fn.move_to.md) — Moves the window to the given logical coordinates.
- [`oldest`](fn.oldest.md) — Gets the window [`Id`] of the oldest window.
- [`open`](fn.open.md) — Opens a new window with the given [`Settings`]; producing the [`Id`] of the new window on completion.
- [`open_events`](fn.open_events.md) — Subscribes to all [`Event::Opened`] occurrences in the running application.
- [`position`](fn.position.md) — Gets the position in logical coordinates of the window with the given [`Id`].
- [`raw_id`](fn.raw_id.md) — Gets an identifier unique to the window, provided by the underlying windowing system. This is not to be confused with [`Id`].
- [`request_user_attention`](fn.request_user_attention.md) — Requests user attention to the window. This has no effect if the application is already focused. How requesting for user attention manifests is platform dependent, see [`UserAttention`] for details.
- [`resize`](fn.resize.md) — Resizes the window to the given logical dimensions.
- [`resize_events`](fn.resize_events.md) — Subscribes to all [`Event::Resized`] occurrences in the running application.
- [`run`](fn.run.md) — Runs the given callback with a reference to the [`Window`] with the given [`Id`].
- [`scale_factor`](fn.scale_factor.md) — Gets the scale factor of the window with the given [`Id`].
- [`screenshot`](fn.screenshot.md) — Captures a [`Screenshot`] from the window.
- [`set_icon`](fn.set_icon.md) — Changes the [`Icon`] of the window.
- [`set_level`](fn.set_level.md) — Changes the window [`Level`].
- [`set_max_size`](fn.set_max_size.md) — Set the inner maximum size of the window.
- [`set_min_size`](fn.set_min_size.md) — Set the inner minimum size of the window.
- [`set_mode`](fn.set_mode.md) — Changes the [`Mode`] of the window.
- [`set_resizable`](fn.set_resizable.md) — Set the window to be resizable or not.
- [`set_resize_increments`](fn.set_resize_increments.md) — Set the window size increment.
- [`show_system_menu`](fn.show_system_menu.md) — Shows the [system menu](https://en.wikipedia.org/wiki/Common_menus_in_Microsoft_Windows#System_menu) at cursor position.
- [`size`](fn.size.md) — Gets the window size in logical dimensions.
- [`toggle_decorations`](fn.toggle_decorations.md) — Toggles the window decorations.
- [`toggle_maximize`](fn.toggle_maximize.md) — Toggles the window to maximized or back.
