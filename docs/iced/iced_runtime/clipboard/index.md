---
title: "clipboard"
kind: module
path: "iced_runtime::clipboard"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/clipboard/index.md"
docs_url: "https://docs.iced.rs/iced_runtime/clipboard/index.html"
source_file: "src/iced_runtime/clipboard.rs"
source_lines: "1-112"
---

# `iced_runtime::clipboard`

Access the clipboard.

## Enums

- [`Action`](enum.Action.md) — A clipboard action to be performed by some [`Task`](../task/struct.Task.md).

## Functions

- [`read`](fn.read.md) — Read the given [`Kind`] of [`Content`] from the clipboard.
- [`read_files`](fn.read_files.md) — Read the current file paths of the clipboard.
- [`read_html`](fn.read_html.md) — Read the current HTML contents of the clipboard.
- [`read_image`](fn.read_image.md) *(image)* — Read the current `Image` of the clipboard.
- [`read_text`](fn.read_text.md) — Read the current text contents of the clipboard.
- [`write`](fn.write.md) — Write the given [`Content`] to the clipboard.
