# iced documentation (Markdown)

A structured, link-checked Markdown conversion of the published rustdoc site
<https://docs.iced.rs/iced/>, built for AI agents and offline search.

- **Version:** `0.15.0-dev`
- **Upstream commit:** `cd04b74ca7073766f654535d6e72c452ef749d07`
- **Crates:** 16
- **Doc pages:** 3313
- **Source pages:** 275
- **Re-export stubs:** 1314

## Start here

- [`index.md`](index.md) — full table of contents.
- [`llms.txt`](llms.txt) — compact map for LLM context windows.
- [`llms-full.txt`](llms-full.txt) — every doc page concatenated (large).
- [`manifest.json`](manifest.json) — machine-readable page list.

## Layout

```
markdown/
  <crate>/
    index.md                  # crate root
    <module>/
      index.md                # module: items + one-line summaries
      struct.Container.md     # one file per item
      fn.center.md
      trait.Catalog.md
  src/                        # annotated Rust source
```

File names mirror rustdoc exactly (`struct.*`, `enum.*`, `trait.*`, `fn.*`,
`type.*`, `constant.*`, `static.*`, `macro.*`), so `struct.Container.html`
always maps to `struct.Container.md`.

## Conventions

Every page starts with YAML frontmatter:

```yaml
---
title: "Container"
kind: struct                 # crate | module | struct | enum | trait | function | type | ...
path: "iced::widget::Container"
crate: "iced"
module: "iced::widget"
file: "iced/widget/struct.Container.md"
docs_url: "https://docs.iced.rs/iced/widget/struct.Container.html"
source_file: "src/iced_widget/container.rs"
source_lines: "258-260"
---
```

Body conventions:

- `# \`crate::module::Item\`` is always the first heading.
- Declarations are fenced ```rust blocks (exact rustdoc signature).
- `## Methods`, `## Trait Implementations`, `## Blanket Implementations`,
  `## Fields`, `## Variants`, `## Implementors` group members.
- A member heading is its bare name; its exact signature follows in a ```rust block.
- Internal links are relative `.md` links; rustdoc anchor fragments are dropped.
- Re-exported items keep their page as a short stub pointing at the canonical path
  (frontmatter `canonical`).

## License

Documentation text comes from the iced project (MIT). See the upstream repository
at <https://github.com/iced-rs/iced>.
