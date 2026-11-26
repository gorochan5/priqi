# 2026-11-26 — Large Dataset (11,636+ Rows) UI Lag, Eager Boot Load & Blocking CSV Import

## 1. Context & Symptom

- **Observed behavior:** Importing and interacting with a dataset of **11,636 rows** caused severe UI sluggishness, slow startup/switching, and all rows being eagerly loaded into RAM across every `priqi.exe` instance.
- **Deployment context:** 5–10 concurrent Windows Server 2022 RDS instances on VMware (`srv-ad-rds`), where many instances sit idle.

---

## 2. Investigation & Evidence

1. **Unvirtualized table (`src/core/datasets/widgets/dataset_table.rs`):** Every `view()` constructed checkboxes, dividers, `mouse_area`s, and `text` widgets for all 11,636 rows (~300,000+ widgets per frame) despite fixed `41.0px` (`40px` row + `1px` divider) row height.
2. **Per-frame filtering & selection scan:** `dataset_table.rs` called `filtered_row_indices(table, &state.search_query)` and scanned all matching rows for `all_selected` / `some_selected` on every `view()`.
3. **UI-thread CSV import & `text_editor` bloat (`src/core/widgets/import_modal/update.rs`, `src/core/datasets/update.rs`):** Uploading a CSV file read the entire file into `text_editor::Content` (shaping 11,636+ lines in `cosmic-text`) and parsed it synchronously on the UI thread.
4. **Full-table cloning & eager SQLite loading (`src/core/db/mod.rs`, `src/core/datasets/models.rs`, `src/core/update.rs`):**
   - `Db::load_datasets()` loaded and CSV-parsed every dataset in `priqi.db` at boot.
   - Every cell edit or dataset rename cloned the entire `TableData` and serialized the full CSV on the async worker thread.

---

## 3. Root Cause

Constructing O(N) Iced widgets per frame for offscreen rows, re-filtering all rows on every `view()`, loading multi-megabyte CSVs into `text_editor::Content`, eagerly loading all datasets into RAM at boot, and deep-cloning `TableData` on saves caused CPU/memory scaling proportional to total dataset rows.

---

## 4. Resolution & Takeaways

### Changes Applied
- **Virtual Scrolling (`src/core/datasets/widgets/dataset_table.rs`, `src/core/datasets/models.rs`, `src/core/datasets/update.rs`):**
  - Added `.on_scroll(...)` on `TABLE_ROWS_SCROLLABLE_ID` emitting `Message::TableScrolled { y, height }`, handled via an O(1) fast path at the top of `Message::Datasets` in `src/core/update.rs` (skipping editor commits, DB checks, and preview scheduling).
  - Rendered only visible rows (`start_pos..end_pos` with `OVERSCAN_ROWS = 6` at `ROW_STRIDE = 41.0px`) bounded by top and bottom `space().height(...)` spacers.
  - Added `cached_filtered_indices: Option<Vec<usize>>` (rebuilt only on search/table/row mutations; `None` when not searching) and O(1) `selection_flags()` when not searching.
- **Non-Blocking CSV Import (`src/core/widgets/import_modal/{models,update,view}.rs`, `dropzone.rs`, `src/core/datasets/update.rs`):**
  - Bumped max file size to 25 MB; for `ImportKind::Datasets`, `load_file_into_state` validates metadata without reading bytes into `text_editor::Content`, showing formatted file size in `dropzone.rs` and disabling modal actions while `is_importing` is `true`.
  - Offloaded CSV file reading and `parse_csv_to_table` (both File and Paste tabs) onto `tokio::task::spawn_blocking` returning `Message::CsvImportFinished`.
- **Lazy SQLite Loading & CoW `Arc<TableData>` (`src/core/db/mod.rs`, `src/main.rs`, `src/core/models.rs`, `src/core/update.rs`, `src/core/datasets/models.rs`):**
  - Replaced `load_datasets` with `load_dataset_metadata()` (boot loads only `(id, name)` + the first dataset's table) and `load_dataset_table(id)` (parses CSV on `spawn_blocking` with `dataset_load_generation` stale-result guard).
  - Switched `State::tables` to `HashMap<String, Arc<TableData>>` using `Arc::make_mut` on mutations, `evict_inactive_tables()` on dataset switches, `spawn_blocking` CSV serialization in `save_dataset`, and lightweight `rename_dataset` SQL updates (`UPDATE datasets SET name = ?1 ...`).

### Verification
- `cargo clippy --no-deps --all-targets -- -D warnings` (via `rust-mcp-server`) passed with **0 errors and 0 warnings**.
