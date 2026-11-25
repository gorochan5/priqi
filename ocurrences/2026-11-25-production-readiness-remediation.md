# Production Readiness Remediation: Fixing Data Loss, UI Freezes, and Stubbed Printing in `priqi_iced`

**Date:** 2026-11-25  
**Project:** `priqi_iced` (`iced 0.15.0-dev`, `sqlx 0.9` SQLite, `labelize 1.9`)

---

## 1. Context & Symptom

Before shipping `priqi_iced` to production, a full-codebase audit ([25-11-2026_fix.md](../25-11-2026_fix.md)) uncovered multiple critical reliability, performance, and feature gaps:
- **Silent data loss:** Importing CSVs with extra/ragged columns worked in memory but vanished on restart; deleting a template caused subsequent IDs to collide and overwrite existing SQLite records; renaming an unselected template saved the wrong template; pending cell edits were lost when switching datasets or closing the app.
- **UI freezes & GPU texture thrashing:** `icons::branding()` allocated a new `Handle::from_bytes` (`Id::unique()`) every frame; global mouse clicks and window unfocus events triggered unconditional SQLite writes and synchronous ZPL rasterization; printing and file reads ran synchronously on the GUI thread.
- **Stubbed print & PDF backends:** `"Térmica 80mm"` and `"Laser Escritório"` wrote a dummy `priqi_print_output.zpl` file to CWD, and `"Salvar em PDF"` wrote `output_labels/etiqueta_N.png` while ignoring `copies` and label rotation.
- **Missing production workflows:** No Windows Print Spooler integration, no real multi-page PDF output, no inline dataset CRUD (rename/delete dataset, delete rows/columns), no configurable label dimensions/DPI, and no CSV `;` delimiter auto-detection.

---

## 2. Investigation & Evidence

Four parallel specialist audits (verified against `docs/iced/` and `cargo clippy` / `cargo check`) traced each failure mode to its exact source:
- **CSV persistence failure:** `parse_csv_to_table` (`src/core/datasets/update.rs`) enabled `.flexible(true)` and padded short rows, leaving wide rows longer than `headers.len()`, whereas `table_to_csv_strings` (`src/core/db/mod.rs`) used `flexible(false)` and failed with `UnequalLengths` while `spawn_save_dataset` discarded errors via `let _ =`.
- **Unconditional DB writes & ZPL renders:** `src/core/update.rs` spawned `spawn_save_dataset` / `spawn_save_template` and synchronous `refresh_all_template_previews` / `refresh_preview` on every `Message::Datasets`, `Message::Templates`, and `Message::Editor(Unfocus)` event without checking dirty state.
- **Iced 0.15 widget bugs:** `DashedBorderCanvas` cached initial gray geometry in `canvas::Cache` without invalidation; `ZplParser` failed to backtrack on unclosed `[` brackets; `TextEditor` received `f32::INFINITY` height inside `Scrollable`; `preview_frame.rs` decoded PNGs to rotate pixels on CPU instead of using `Image::rotation`.

---

## 3. Root Cause

The prototype lacked:
1. **Dirty-state boundaries** between UI interaction messages and persistence/rasterization side effects.
2. **Uniform data normalization** and collision-free ID allocation across SQLite and in-memory state.
3. **Platform print/dialog integration** (`winspool.drv` RAW spooling, `comdlg32.dll` `GetSaveFileNameW`, and multi-page PDF generation) and **async task offloading** (`tokio::task::spawn_blocking`).

---

## 4. Resolution & Takeaways

### Implemented Fixes & Features
1. **Core, DB, Renderer & Printer (`src/core/{db/mod.rs,renderer.rs,printer.rs,icons.rs,update.rs,view.rs,models.rs}`, `src/main.rs`):**
   - Configured SQLite WAL mode, `busy_timeout(5s)`, `synchronous(NORMAL)`, idempotent `width_mm`/`height_mm`/`dpi` columns on `templates`, `delete_dataset`, and per-row resilient dataset loading.
   - Replaced `Printer` enum with `PrinterOption::OsPrinter(String)` / `PrinterOption::Pdf`, enumerating installed Windows printers via `EnumPrintersW` and spooling RAW ZPL via `OpenPrinterW`/`StartDocPrinterW`/`WritePrinter` on `spawn_blocking`.
   - Implemented modal `GetSaveFileNameW` + pure-Rust multi-page PDF 1.4 export honoring `copies`, `rotation`, `LabelRenderOptions`, and all parsed ZPL `labels`.
   - Cached `icons::branding()` `Handle` with `std::sync::LazyLock`, rewrote `substitute_template` as a single-pass scanner, initialized `tracing_subscriber`, added `window::close_requests()` + `Escape` handling, and gated `iced/hot` behind an optional Cargo feature.
2. **Datasets, Templates & Import Modal (`src/core/{datasets,templates,widgets/import_modal}/**`):**
   - Normalized ragged wide/short CSV rows (`parse_csv_to_table`), auto-detected `,`/`;`/`\t`, fixed `ImportPressed` to always create a new dataset on paste, fixed `SelectAllRows` search filtering, wired `SelectRow`/`CancelEditing`, added `.clip(true)` + `.ellipsis(Ellipsis::End)`, and implemented inline dataset CRUD (rename/delete dataset, delete selected rows, delete column).
   - Fixed template/dataset ID collisions using max numeric suffix, fixed `SaveRename` persistence for unselected cards, rendered `file_error` across both import modal tabs, fixed `DashedBorderCanvas` (`type State = ()`), and synchronized `paste_editor` scrolling.
3. **Editor, Controls & Navbar (`src/core/{editor,controls,navbar}/**`):**
   - Added `is_dirty` tracking and cached `line_numbers` in `editor`, fixed `ZplParser` unclosed `[` backtracking and `^FX`/3-letter commands, switched preview rotation to GPU `Image::rotation(Rotation::Solid(...))`, and added `W × H` (mm) + `203/300/600` DPI controls in the Preview toolbar.
   - Aligned `controls` without `+/-` steppers, fixed the `toggle_button` border seam and editor bottom inset, and updated `navbar` with `State { is_maximized }`, centered controls, red `exit` hover, fixed `version.rs` badge, and draggable `branding`.

### Verification
- `cargo check --all-targets --locked --quiet`: **0 errors, 0 warnings**.
- `cargo clippy --no-deps --all-targets --locked --quiet`: **0 errors, 0 warnings**.

### Takeaways
- Always gate persistence and heavy rasterization behind explicit dirty flags rather than coarse message-category matches.
- In Iced 0.15, cache `Handle::from_bytes` in `LazyLock` for static images and use `type State = ()` on `canvas::Program` when colors/properties change dynamically at constant bounds.
