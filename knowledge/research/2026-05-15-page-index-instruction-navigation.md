---
id: research_2026_05_15_page_index_instruction_navigation
kind: research-note
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-15
review_after: 2026-06-15
research_gate: R2
---

# PageIndex-style instruction navigation

## Question

How should Punk organize `.punk` project instructions and module instructions so
humans and agents can navigate them without bloated generated pages, hidden
truth stores, vector DBs, or giant prompt blobs?

## Source checked

- `https://github.com/VectifyAI/PageIndex`
- Observed HEAD: `7592163e2a376b3917181fff9ac1858dc5daa2c6`
- README claims: PageIndex builds a hierarchical tree structure, similar to a
  table of contents, and performs reasoning-based tree search instead of vector
  similarity search or artificial chunking.
- README feature claims: no vector DB, no chunking, traceable page/section
  references, context-aware retrieval, human-like navigation.
- `run_pageindex.py` supports both PDF and Markdown inputs and writes a
  `*_structure.json` tree into `results/`.
- The Markdown path uses heading hierarchy as the structural input.

## Findings

PageIndex is relevant as an indexing pattern, not as an immediate Punk runtime
dependency.

The useful pattern for Punk is:

- keep source instructions as small, natural pages;
- build a hierarchical page index over those pages;
- route humans and agents through the index;
- keep each generated/HTML view thin and link-based;
- preserve traceable references to source instruction pages;
- avoid vector DBs, opaque chunking, and hidden memory as truth.

The risky parts for Punk are:

- PageIndex generation can depend on LLM calls and API keys;
- PageIndex examples include PDF/OCR/service paths that are outside Punk active
  core;
- generated summaries must not become source of truth;
- retrieval reasoning must remain advisory and inspectable.

## Punk interpretation

Use a PageIndex-style instruction tree inside initialized projects:

```text
.punk/
  README.md
  project.toml
  instructions/
    INDEX.md
    pages/
      getting-started.md
      layout.md
      init.md
      modules.md
      authority.md
    modules/
      <module-id>/
        INDEX.md
        pages/
          overview.md
          commands.md
          receipts.md
          boundaries.md
  views/
    instructions/
      page-index.json
      index.html
```

`README.md`, `instructions/INDEX.md`, and optional HTML are entrypoints, not
complete manuals.

Instruction source pages are truth candidates. Generated page indexes and HTML
views are derived, rebuildable, and advisory.

## Recommended boundary

Adopt PageIndex as a design pattern:

- hierarchical tree of instruction pages;
- natural sections over artificial chunks;
- explicit refs from nodes to source pages;
- no vector DB;
- no hidden content store;
- no raw prompt or transcript storage;
- no module authority escalation.

Do not add the PageIndex package, Python runtime, LLM calls, OCR, cloud API,
MCP, or vector/RAG runtime in the first Punk slice.

## Open questions

- Should the first generated index be Markdown-only, JSON-only, or both?
- Should `punk init` create the full instruction source tree immediately, or
  only a thin `.punk/README.md` plus `.punk/instructions/INDEX.md`?
- Should module instructions live under `.punk/instructions/modules/<id>/` or
  `.punk/modules/<id>/instructions/` once module manifests exist?
- Should `punk guide`, `punk instructions`, or `punk help` own user-facing
  navigation?

## Recommendation

Start with a deterministic local PageIndex-style instruction index model.

First implementation slice should avoid LLM summarization. It can derive a tree
from Markdown headings and explicit frontmatter:

- `id`
- `title`
- `kind`
- `status`
- `authority`
- `source_ref`
- `module_id`
- `children`

Later slices may add summaries, search routing, and optional adapter-backed
retrieval receipts.
