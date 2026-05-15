---
id: idea_2026_05_15_punk_instruction_page_index
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-15
review_after: 2026-06-15
---

# Punk instruction page index

## Intent

Every initialized Punk project should have its own local instruction surface
under `.punk/`.

The instruction surface should be useful to humans and agents without becoming
a giant manual, generated HTML blob, hidden memory store, or second source of
truth.

## Shape

Use a PageIndex-style hierarchy:

```text
.punk/README.md
.punk/instructions/INDEX.md
.punk/instructions/pages/*.md
.punk/instructions/modules/<module-id>/INDEX.md
.punk/instructions/modules/<module-id>/pages/*.md
.punk/views/instructions/page-index.json
.punk/views/instructions/index.html
```

Rules:

- `.punk/README.md` is the shortest entrypoint.
- `instructions/INDEX.md` is the human table of contents.
- `instructions/pages/*.md` hold focused instruction pages.
- module instruction trees are nested under the instruction hierarchy.
- `views/instructions/page-index.json` is derived and rebuildable.
- `views/instructions/index.html` is optional, thin, and link-only.
- generated views do not duplicate full source pages.

## Page index node

Candidate node shape:

```toml
id = "punk.instructions.init"
title = "Init"
kind = "instruction-page"
status = "active"
authority = "advisory"
source_ref = ".punk/instructions/pages/init.md"
module_id = "core"
summary = "How to initialize Punk in a project."
children = []
```

`summary` is optional in the first slice. If summaries are generated later,
they remain advisory and must reference source pages.

## Module instruction rule

A module may add instructions, but it cannot own truth.

Module instructions may explain:

- module purpose;
- local layout;
- commands;
- receipts;
- boundaries;
- policy requirements;
- inspect output;
- failure handling.

Module instructions may not:

- write final decisions;
- bypass core Punk laws;
- create hidden truth stores;
- treat generated summaries as truth;
- make external side effects active by documentation alone.

## First useful slice

Implement deterministic instruction source and page-index scaffolding:

1. Define the instruction page index model.
2. Update `punk init` to create thin instruction entrypoints only.
3. Add `punk instructions inspect` or `punk guide` later to render the tree.

Do not add PageIndex as a dependency in the first slice.

Do not add LLM summarization, OCR, MCP, vector DB, cloud sync, module host,
PubPunk activation, adapter behavior, or external publishing.
