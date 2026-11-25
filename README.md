# Priqi

**Priqi** is a native desktop application for designing, previewing, data-binding, and printing **ZPL (Zebra Programming Language)** thermal labels. Built with **Rust** and **Iced (`0.15.0-dev`)**, it combines a syntax-highlighted ZPL editor, live rasterized previews, a spreadsheet-like dataset manager for variable data printing, direct Windows RAW print spooling, and a pure-Rust multi-page PDF exporter.

---

## Features

### ZPL Code Editor & Live Preview
- **Syntax-Highlighted ZPL Editor:** Built-in code editor with dynamic line numbering, ZPL command highlighting (`^XA`, `^FO`, `^FD`, `^FX`, etc.), and dynamic variable tag highlighting (`[COLUMN_NAME]`).
- **Multi-Label Live Preview:** Rasterizes ZPL templates in real time using `labelize`, rendering separate cards for each selected dataset row (and multiple `^XA...^XZ` blocks within a single template).
- **Physical Dimensions & Density Controls:** Configure label width and height in millimeters (default `100 × 150 mm`) and print density (`203`, `300`, or `600` DPI) per template.
- **Rotation & Inspection:** GPU-accelerated 90° step rotation (`0°`, `90°`, `180°`, `270°`) and full-window card maximization with keyboard/button navigation.

### Template Management
- **Template Library:** Create, duplicate, inline-rename, and delete ZPL templates with live miniature previews.
- **File & Clipboard Import:** Import `.zpl` or `.txt` templates via drag-and-drop onto the window/modal dropzone or by pasting raw ZPL code into the syntax-highlighted import modal.

### Dataset Management & Variable Data Binding
- **Interactive Tabular Editor:** Create, rename, and delete datasets; add or remove rows and columns; double-click cells or column headers to edit inline.
- **CSV Import & Auto-Detection:** Import `.csv` or `.txt` files via drag-and-drop or clipboard paste with automatic delimiter detection (`,`, `;`, `\t`) and ragged-row normalization.
- **Filtering & Multi-Row Selection:** Filter rows in real time via search query, toggle individual rows, range-select with `Shift + Click`, or select all filtered rows for batch printing.
- **Safe Tag Interpolation:** Single-pass `[COLUMN_NAME]` substitution engine that prevents cross-column re-substitution and strips ZPL control characters (`^`, `~`) from dataset values to prevent ZPL command injection.

### Printing & Multi-Page PDF Export
- **Windows RAW Print Spooler (`winspool.drv`):** Automatically enumerates local and network printers via `EnumPrintersW` and dispatches RAW ZPL payloads (`OpenPrinterW`, `StartDocPrinterW`, `WritePrinter`) on background threads, injecting `^PQ{copies}` before each `^XZ` terminator when not explicitly present.
- **Multi-Page PDF 1.4 Export:** Renders all resolved rows, label blocks, rotations, and copy counts into a standards-compliant multi-page PDF document (sized accurately in PostScript points from the template's millimeter dimensions) via the native Windows `GetSaveFileNameW` dialog.

### Local SQLite Persistence
- **Automatic Persistence:** Stores all templates (including physical dimensions and DPI) and datasets in a local `priqi.db` SQLite database configured with `WAL` journaling, `NORMAL` synchronization, and automatic idempotent schema migrations.

---

## Tech Stack

| Layer | Crate / Technology | Purpose |
| :--- | :--- | :--- |
| **GUI Framework** | [`iced`](https://github.com/iced-rs/iced) `0.15.0-dev` | Elm-inspired MVU desktop UI (`image`, `canvas`, `tokio` features) |
| **ZPL Engine** | [`labelize`](https://crates.io/crates/labelize) `1.9.0` | ZPL parsing and PNG label rasterization |
| **Image Processing** | [`image`](https://crates.io/crates/image) `0.25` | PNG decoding and RGB/RGBA transformations for PDF & previews |
| **Database** | [`sqlx`](https://crates.io/crates/sqlx) `0.9.0` | Async SQLite connection pool, queries, and migrations |
| **Async Runtime** | [`tokio`](https://crates.io/crates/tokio) `1.54` | Background DB I/O and `spawn_blocking` print/render tasks |
| **Data Parsing** | [`csv`](https://crates.io/crates/csv) `1.4`, [`serde`](https://crates.io/crates/serde) `1.0` | Flexible CSV parsing and dataset serialization |
| **Diagnostics** | [`tracing`](https://crates.io/crates/tracing) `0.1`, `tracing-subscriber` `0.3` | Structured logging and error diagnostics |
| **OS Integration** | Win32 FFI (`winspool`, `comdlg32`, `user32`, `ole32`) | Native printer enumeration, RAW ZPL spooling, and Save File dialog |

---

## Project Structure

```text
priqi_iced/
├── Cargo.toml                  # Package manifest (Rust 2024 edition)
├── AGENTS.md                   # AI assistant guidelines & offline docs instructions
├── docs/iced/                  # Read-only offline snapshot of iced 0.15.0-dev API docs
├── ocurrences/                 # Engineering incident & remediation logs
├── resources/
│   ├── branding.png            # Navbar branding asset
│   ├── icon.ico / icon.png     # Application window icons
│   └── lucide/                 # Embedded Lucide icon font (lucide.ttf & CSS mapping)
├── scripts/
│   └── gen-icons.ps1           # PowerShell generator for src/core/icons.rs
└── src/
    ├── main.rs                 # Application entrypoint, window settings & global subscriptions
    └── core/
        ├── mod.rs              # Core module registry
        ├── models.rs           # Root application State and Message definitions
        ├── update.rs           # Root MVU update loop, dirty-state persistence & task orchestration
        ├── view.rs             # Root layout composition and error banners
        ├── icons.rs            # Auto-generated Lucide icon helpers and cached branding handle
        ├── renderer.rs         # ZPL-to-PNG rasterizer and [COLUMN] template substitution engine
        ├── printer.rs          # Win32 RAW print spooler, printer enumeration & PDF 1.4 builder
        ├── config/             # Theme palette, layout constants & typography helpers
        ├── db/                 # SQLite pool initialization, schema migrations & CRUD queries
        ├── navbar/             # Custom frameless window titlebar (drag, minimize, maximize, exit)
        ├── controls/           # Printer selector dropdown, copies input & print action bar
        ├── editor/             # Code tab (ZPL editor + syntax highlighter) & Preview tab
        ├── templates/          # Template cards carousel, inline rename & thumbnail previews
        ├── datasets/           # Dataset selector, search bar & editable spreadsheet table
        └── widgets/            # Shared custom widgets (dot grid canvas, drag-and-drop import modal)
```

---

## Variable Data Binding Syntax

Inside any ZPL template, reference dataset columns by wrapping the exact column header name in square brackets:

```zpl
^XA
^CF0,40
^FO50,50^FDProduto: [PRODUTO]^FS
^CF0,30
^FO50,110^FDSKU: [SKU]^FS
^FO50,160^BY2,3,80^BCN,80,Y,N,N^FD[EAN]^FS
^XZ
```

When a dataset is loaded:
1. Each `[COLUMN_NAME]` tag is replaced in a single pass with the corresponding cell value from each selected row (or the first row during live preview if no rows are explicitly checked).
2. Reserved ZPL control characters (`^` and `~`) inside dataset cell values are stripped automatically so user data cannot inject arbitrary ZPL commands.
3. Unmatched or unclosed brackets (e.g., literal `[123]`) are preserved as-is.

---

## Database Schema (`priqi.db`)

Priqi automatically creates and migrates `priqi.db` in the working directory on startup:

- **`templates`**:
  - `id` (`TEXT PRIMARY KEY`)
  - `name` (`TEXT NOT NULL`)
  - `zpl` (`TEXT NOT NULL`)
  - `width_mm` (`REAL NOT NULL DEFAULT 100.0`)
  - `height_mm` (`REAL NOT NULL DEFAULT 150.0`)
  - `dpi` (`INTEGER NOT NULL DEFAULT 203`)
  - `created_at`, `updated_at` (`DATETIME`)
- **`datasets`**:
  - `id` (`TEXT PRIMARY KEY`)
  - `name` (`TEXT NOT NULL`)
  - `headers` (`TEXT NOT NULL` — serialized CSV header record)
  - `rows` (`TEXT NOT NULL` — serialized CSV data records)
  - `created_at`, `updated_at` (`DATETIME`)

---

## Development & Diagnostics

### Prerequisites
- **Rust Toolchain:** Rust `2024` edition (`rustc` / `cargo`).
- **Operating System:** Windows is supported for native `winspool.drv` RAW printing and `comdlg32.dll` file dialogs (non-Windows targets compile with graceful fallbacks).

### Type-Checking & Linting
Use `cargo check` and `cargo clippy` to validate changes against the pinned dependencies:

```sh
cargo check --all-targets --locked
cargo clippy --no-deps --all-targets --locked
```

### Optional Features
- `hot`: Enables `iced/hot` for hot-reloading support during UI development:
  ```sh
  cargo check --features hot
  ```

### Regenerating Icon Bindings
If `resources/lucide/lucide.css` or `lucide.ttf` is updated, regenerate [`src/core/icons.rs`](src/core/icons.rs) from the repository root:

```powershell
pwsh scripts/gen-icons.ps1
```

### Offline Iced Documentation
This repository includes a complete offline Markdown snapshot of the `iced 0.15.0-dev` API documentation under [`docs/iced/`](docs/iced/index.md), pinned to the exact commit in `Cargo.lock`. See [`AGENTS.md`](AGENTS.md) for search patterns and usage conventions.
