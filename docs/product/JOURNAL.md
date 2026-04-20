---
id: docs_product_journal
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - journal-surface
  - origin-journal-model
  - journal-artifact-structure
related_docs:
  - docs/product/PUBLIC-NARRATIVE.md
  - docs/product/PROJECT-MEMORY.md
related_adrs: []
supersedes: []
superseded_by: null
---

# Journal

## Purpose

The Journal is the public origin chronicle for `punk`.

It records the experiments, failures, artifacts, and decisions that led to Punk, while preserving the same discipline as the rest of the project: repo-tracked memory, source-backed claims, honest status labels, and no hidden second source of truth.

The external surface becomes discipline: the public story is not marketing copy detached from the project. It is a structured view over project memory.

## Decision

Adopt a repo-backed Journal surface for the site.

Working public name:

```text
Journal
```

Primary page title:

```text
The Road to Punk
```

Internal concept name:

```text
Origin Journal
```

This is not a conventional blog. It is not a reverse-chronological feed, a CMS, or a magazine flipbook. It is a chronological public memory surface with a main spine, side branches, artifacts, status labels, and source references.

## Scope classification

This belongs to:

- `Phase -1 — Public narrative seed`
- the Public Narrative Plane
- the site / brand surface
- manual project memory and public storytelling

It does not belong to the active runtime core.

It is not:

- `active-core` CLI behavior
- PubPunk automation
- external publishing integration
- metrics scraping
- content scheduling
- a social growth engine

Future PubPunk automation may later adopt the Journal structure. PubPunk must not create a separate content truth.

## Laws and risks touched

Relevant laws / principles:

- One project memory model; no hidden second source of truth.
- Project memory is explicit and authority-tagged.
- Raw ideas are not implementation truth by default.
- Public build artifacts are project memory.
- Manual publication is allowed before PubPunk automation.
- Public claims must be supported by canonical knowledge or framed as opinion.
- Parked or future-only surfaces must not be described as current operator paths.

Main risks:

- turning the Journal into a generic blog
- overclaiming product readiness
- presenting legacy experiments as current Punk behavior
- creating a site-only content source outside `publishing/`
- making a visually impressive but hard-to-navigate graph
- letting comics become decoration instead of memory-bearing artifacts

## Recommendation

Adopt the Journal concept now, but keep the first implementation small and manual.

The first version should be a static `/journal` page backed by repo-tracked Markdown/frontmatter under `publishing/journal/`.

It should support:

- eras
- short entries
- source references
- artifact references
- thread filters
- status labels
- comic/image entries as optional media
- deep links to each entry

It should not require:

- a CMS
- search infrastructure
- automated publication
- external APIs
- metrics collection
- complex graph rendering

## Product model

The Journal explains how Punk emerged.

It should answer five questions repeatedly:

```text
What did we believe?
What did we try?
What broke?
What survived into Punk?
What was rejected, parked, or deferred?
```

The most important recurring field is:

```text
what_survived
```

Without it, the Journal becomes nostalgia. With it, the Journal becomes architectural memory.

## Information architecture

### Canonical public narrative objects

The Journal should not replace existing Public Narrative objects. It adds a more structured chronology layer.

```text
Story
  durable narrative arc
  example: why Punk exists

Journal Era
  a period in the origin chronology
  example: Signum, SpecPunk, Reset

Journal Entry
  a concrete historical point inside an era
  example: first contract-first experiment

Post
  a concrete channel-ready draft or publication text

Publication Receipt
  proof that a post was published to a public channel

Metrics Snapshot
  manual or automated performance record
```

### Suggested storage

```text
publishing/
  journal/
    README.md
    eras/
      2025-05-raw-agents.md
      2025-xx-vibe-coding.md
      2025-xx-signum.md
      2026-xx-specpunk.md
      2026-04-reset.md
      2026-04-punk.md
    entries/
      2025-05-001-first-raw-agent.md
      2025-xx-002-contracts-before-code.md
    assets/
      comics/
      screenshots/
      diagrams/
  _templates/
    journal-era.md
    journal-entry.md
  _schema/
    journal-entry.schema.json
```

A site page should render from these repo artifacts rather than from a separate site-only content store.

## UX model

The core UX should be:

```text
linear spine + side branches + thread filters
```

The main timeline is chronological. Side branches show experiments, artifacts, failures, comics, screenshots, diagrams, and links.

Avoid a full graph as the primary navigation. A graph may look interesting, but it can become visually noisy and hard to use. The default experience should remain readable and navigable.

### `/journal` page structure

Suggested first page structure:

```text
1. Hero
   The Road to Punk
   A repo-tracked chronology of the experiments, failures, and systems that led to Punk.

2. Current state capsule
   early-stage
   core-first
   architecture/design stage where relevant
   modules later
   public build from day zero

3. Era map
   Raw Agents
   Vibe Coding
   Signum
   SpecPunk
   Reset / Realignment
   Punk

4. Sticky navigation
   by era
   by thread
   by entry type

5. Timeline spine
   short cards grouped by era

6. What survived matrix
   old experiment -> adopted Punk principle

7. Artifact shelf
   READMEs
   screenshots
   diagrams
   comics
   receipts

8. CTA
   follow the build
   read the roadmap
   open public artifacts
```

### Navigation modes

The Journal should support at least two reading modes:

```text
By Era
  Raw Agents
  Vibe Coding
  Signum
  SpecPunk
  Reset
  Punk

By Thread
  contracts
  proof
  project memory
  evals
  gate
  agent drift
  public build
  UX / shell
  research
  failures
```

A reader should always understand:

```text
where am I in the chronology?
what era is this?
is this current, legacy, parked, retired, or speculative?
what did this lead to?
what source backs this claim?
```

## Visual direction

The Journal should extend the current landing visual language rather than invent a separate style.

Use:

- dark technical background
- restrained neon/magenta accents
- mono metadata
- thin bordered panels
- terminal / receipt / proof aesthetics
- artifact cards
- timeline rails
- source refs
- status pills
- the Punk Porcupine mascot

Avoid:

- generic startup blog layout
- NFT/cyberpunk overload
- cute cartoon branding
- page-flip magazine UI
- heavy animation as primary navigation
- dense long-form essay walls by default

## Mascot role

The mascot should not be decoration only.

Role:

```text
guide / witness / state marker
```

The same mascot can appear in different narrative states:

```text
Raw Agents
  unbounded, noisy, unstable

Signum
  contract-first, proof-marked

SpecPunk
  branching, orchestration-heavy

Reset / Realignment
  cutting scope, removing overgrowth

Punk
  bounded, disciplined, core-first
```

The mascot should help orient the reader, not replace the content.

## Comics and graphics

Comics are supported as a first-class entry media type, but the whole Journal should not become a comic-only UI.

Recommended comic format:

```text
1-4 panels
one idea per comic
short caption
source/artifact refs below
```

Each comic entry still needs the Journal memory frame:

```text
what we believed
what happened
what changed
what survived into Punk
```

## Era model

Suggested era frontmatter:

```yaml
---
id: signum
title: Signum
date_range: "TBD"
status: legacy-source
thesis: "TBD"
what_broke: "TBD"
what_survived:
  - contract-first work
  - proofpack direction
  - audit loop
source_refs:
  - "TBD"
---
```

Recommended era statuses:

```text
legacy-source
current-origin
reset-point
current-punk
future-context
```

These are narrative statuses, not replacements for crate/capability statuses such as `active-core`, `incubating`, `parked`, and `retired`.

## Entry model

Suggested entry frontmatter:

```yaml
---
id: 2025-xx-contracts-before-code
date: "TBD"
era: signum
type: milestone
threads:
  - contracts
  - proof
status: adopted-principle
claim_level: canonical
public_state: draft
artifact_refs:
  - "TBD"
source_refs:
  - "TBD"
related_goals: []
related_knowledge: []
---
```

Recommended entry types:

```text
milestone
experiment
artifact
failure
comic
reset
```

Recommended entry statuses:

```text
adopted-principle
legacy-context
retired-path
parked-idea
speculative-note
current-direction
```

Recommended claim levels:

```text
canonical
advisory
opinion
speculative
```

## Entry body template

```markdown
# Title

One-line thesis.

## What we believed
