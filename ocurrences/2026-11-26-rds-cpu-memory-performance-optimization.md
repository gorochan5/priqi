# 2026-11-26 — High CPU (27.4%) & Memory (199.1 MB) on Windows Server 2022 VMware RDS

## 1. Context & Symptom

- **Observed behavior:** A single `--release` instance of `priqi.exe` consumed **27.4% CPU** (~2.5 vCPUs) while scrolling the ZPL code editor and **199.1 MB RAM** in Windows Task Manager.
- **Target environment (`srv-ad-rds`):**
  - Windows Server 2022 Standard (21H2) x86_64 on **VMware 7,1**
  - AMD EPYC 9354 (9 vCPUs @ 3.25 GHz), 48 GiB RAM (already ~81% utilized)
  - Display adapters: **Microsoft Remote Display Adapter** + **VMware SVGA 3D** (no hardware DX12/Vulkan GPU)
  - Deployment model: **5–10 concurrent RDS user sessions**, with several instances left open and idle when users forget to close `priqi.exe`.

---

## 2. Investigation & Evidence

Three parallel audits (Iced UI/Rendering, Rust Systems/Runtime, Desktop Architecture) plus source verification against the pinned Iced `0.15.0-dev` (`cd04b74`) documentation (`docs/iced/`) revealed:

1. **No background polling loop:** Neither the UI (`src/main.rs`) nor backend (`tokio`/`sqlx`/`printer`) ran recurring timers or `window::frames()` subscriptions.
2. **DX12 WARP software rasterization + 60 FPS `smooth_scroll`:**
   - Default `iced` features enabled `wgpu` + `Backend::Best` + `antialiasing: true` (`Antialiasing::MSAAx4`, `docs/iced/src/iced_wgpu/window/compositor.rs.md:440`).
   - On VMware RDS without a hardware GPU, `wgpu` fell back to **Microsoft Basic Render Driver (DX12 WARP)** — multi-core CPU software rasterization with **no dirty-rect partial redraw** (`docs/iced/src/iced_wgpu/lib.rs.md:408-518`).
   - `Scrollable` defaults to `smooth_scroll: true` (`docs/iced/src/iced_widget/scrollable.rs.md:129`), firing 60 FPS `RedrawRequest::NextFrame` loops on every mouse-wheel notch.
   - `DotGrid` (`src/core/widgets/dot_grid.rs`) emitted 3,600 circles in `layer.triangles` behind the maximized 1920×1080 window, forcing **41.5 MB** full-screen 4x MSAA + resolve targets (`docs/iced/src/iced_wgpu/triangle/msaa.rs.md:199-219`) and a split MSAA render pass on every single scroll frame.
3. **`tiny-skia` CPU SDF shadow trap:**
   - Although `iced_tiny_skia` supports dirty-rect damage tracking (`docs/iced/src/iced_tiny_skia/window/compositor.rs.md:173-212`), any `Shadow` with `color.a > 0.0` intersecting the damage rect triggers a per-pixel CPU `rounded_box_sdf` loop across the entire `shadow_bounds`, ignoring `clip_bounds` (`docs/iced/src/iced_tiny_skia/engine.rs.md:50-142`) — allocating ~4.6 MB and shading ~1.16M pixels per frame for the editor panel alone.
4. **Dedicated GPU texture explosion & synchronous render cascades:**
   - `Handle::from_bytes` routed every label preview to `iced_wgpu::image::cache::Worker::upload`, allocating a dedicated square `Atlas::with_size(width.max(height))` per image (`docs/iced/src/iced_wgpu/image/cache.rs.md:579-584`) — 5.76 MB per sidebar thumbnail (`800×1200`), 12.96 MB at 300 DPI, 64 MB at 600 DPI.
   - `Template::new()` synchronously ran `labelize` during `Db::load_templates()`, only for `DbInitialized` to discard and re-render all templates synchronously on the UI thread. Template selection, renames, dimension edits, and row selection synchronously re-rendered all templates and all selected rows even when `Tab::Preview` was inactive.
5. **Per-instance RDS thread & allocation overhead:**
   - Default `iced` `tokio` executor spawned `num_cpus` (9) worker threads per process (`~90` threads across 10 sessions); `SqlitePool` allowed 5 connections (`~50` SQLite worker threads + default 2 MB/conn cache).
   - `iced::keyboard::listen()` mapped all non-Escape keys to `Message::Nothing`, triggering full `view()` + layout + redraw in `iced_winit` (`docs/iced/src/iced_winit/lib.rs.md:1179-1218`).
   - `sync_can_print()` allocated the full editor document via `content.text()` on every message; `dataset_table.rs` ran `.to_lowercase()` filtering twice per `view()`; `update.rs` cloned `selected_rows: BTreeSet<usize>` on every dataset message; `Cargo.toml` lacked `[profile.release]` tuning.

---

## 3. Root Cause

Running `wgpu` (DX12 WARP) with default 4x MSAA, a full-screen `DotGrid` canvas, 60 FPS `smooth_scroll`, full-resolution `Handle::from_bytes` previews, synchronous O(T + S·L) `labelize` cascades, and uncapped multi-threaded Tokio/SQLite pools on a GPU-less VMware RDS host caused both severe baseline RAM inflation (~199 MB/instance) and multi-core CPU saturation (~27.4%) during editor scrolling.

---

## 4. Resolution & Takeaways

### Changes Applied
- **Renderer & Build (`Cargo.toml`, `src/main.rs`):**
  - Switched `iced` to `default-features = false` with `["tiny-skia", "image-without-codecs", "tokio", "crisp", "web-colors"]`, dropping `wgpu`, `naga`, `ash`, `glow`, `canvas`/`lyon`, and Linux display backends.
  - Configured `.backend(iced::Backend::Software)`, `.antialiasing(false)`, and a custom `SmallTokioExecutor` (`worker_threads(1)`, `max_blocking_threads(2)`, `512 KiB` stack).
  - Replaced `iced::window::icon::from_file_data` with `image::load_from_memory` + `iced::window::icon::from_rgba`.
  - Trimmed `image` (`default-features = false, features = ["png"]`) and `tokio` (`["rt-multi-thread", "rt", "time", "sync"]`), and added `[profile.release]` (`lto = "thin"`, `codegen-units = 1`, `strip = "symbols"`, `panic = "abort"`).
  - Folded `Escape` handling into `iced::event::listen_with`, eliminating `Message::Nothing` redraws, and configured `tracing_subscriber` with `EnvFilter` defaulting to `priqi=info,warn`.
- **`tiny-skia` UI Readiness (`src/core/view.rs`, `src/core/widgets/**`, `editor/`, `controls/`, `templates/`, `datasets/`):**
  - Removed `DotGrid` (`src/core/widgets/dot_grid.rs`) in favor of a flat `#f4f6f5` root background and replaced `DashedBorderCanvas` with a standard rounded container border.
  - Replaced all blurred `Shadow { ... }` styles across panels/cards/dropdowns/modals with `Shadow::default()` and crisp 1px borders so `iced_tiny_skia` never enters its per-pixel CPU SDF shadow loop.
  - Set `.smooth_scroll(false)` on all 8 `scrollable` widgets so wheel scrolling renders 1 crisp dirty-rect frame per notch instead of 60 FPS animations over RDP.
  - Cached `import_modal` line numbers and eliminated duplicate search filtering in `dataset_table.rs`.
- **SQLite & Async Cached Preview Pipeline (`src/core/db/mod.rs`, `src/core/renderer.rs`, `src/core/models.rs`, `src/core/update.rs`, `src/core/templates/**`, `src/core/datasets/**`, `src/core/editor/**`):**
  - Reduced `SqlitePool` to `max_connections(1)`, `PRAGMA cache_size = -512`, `statement_cache_capacity(16)`, and guarded `ALTER TABLE` migrations with `PRAGMA user_version`.
  - Added `render_zpl_to_thumbnail_handle` (capped at 203 DPI, aspect-preserving downscale to 360px, `Handle::from_rgba`) and `render_zpl_all_to_preview_handles` (capped at 203 DPI on-screen, `Handle::from_rgba`; printing/PDF retains full template DPI).
  - Removed eager rendering from `Template::new()`, moved all thumbnail and editor preview rendering to `tokio::task::spawn_blocking` with generation cancellation, bounded `HashMap` caches (64 entries), lazy `Tab::Preview` rendering, and `MAX_PREVIEW_ROWS = 12` cap.
  - Replaced `sync_can_print()`'s `content.text()` with zero-allocation line iteration (`content.lines()`) and replaced `selected_rows` `BTreeSet` cloning with O(1) `selection_version: u64` tracking.

### Verification
- `cargo clippy --no-deps --all-targets -- -D warnings` (via `rust-mcp-server`) passed with **0 errors and 0 warnings**.
- Manual validation command for the user on `srv-ad-rds`:
  ```powershell
  cargo build --release
  ```
  Then measure Task Manager RAM (idle & active) and CPU while scrolling the editor across 1 and 5–10 concurrent RDS instances.

### Takeaways
1. On Windows VMware/RDS hosts without a hardware GPU, `wgpu` silently selects DX12 WARP — combining high GPU-style memory overhead with multi-core CPU rasterization and no dirty-rect support.
2. Adopting `tiny-skia` on RDS only achieves low CPU when paired with eliminating blurred `Shadow`s (which bypass `clip_bounds` in `iced_tiny_skia`), removing full-screen `canvas` layers, and disabling 60 FPS `smooth_scroll`.
3. For multi-user RDS deployments where many instances stay idle, capping per-process Tokio workers (`1`) and SQLite pool connections (`1`) prevents linear OS-thread and cache multiplication.
