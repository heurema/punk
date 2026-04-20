# Documentation Map

Status: proposed active-core process doc  
Last updated: 2026-04-19

## Purpose

This document defines how Punk documentation is organized so the project can stay coherent as docs, ADRs, research notes, ideas, evals, and public narrative grow.

The goal is to avoid duplicate truth, drift, and conflicting claims.

## Principle

Define once. Link elsewhere.

A document should own one kind of truth. Other documents may summarize it briefly, but must point back to the canonical owner.

## Canonical reading order

For product and architecture work, read in this order:

1. `README.md`
2. `docs/product/START-HERE.md`
3. `docs/product/DOCUMENTATION-MAP.md`
4. `docs/product/PUNK-LAWS.md`
5. `docs/product/ARCHITECTURE.md`
6. `docs/product/ROADMAP.md`
7. `docs/product/CRATE-STATUS.md`
8. `docs/product/RESEARCH-GATE.md`
9. `docs/product/RESEARCH-INTAKE.md`
10. `docs/product/TELEMETRY.md`
11. `docs/product/EVAL-PLANE.md`
12. `docs/product/PROJECT-MEMORY.md`
13. `docs/product/PUBLIC-NARRATIVE.md`
14. `docs/adr/`
15. `knowledge/research/`
16. `knowledge/ideas/`
17. `work/goals/` and `work/reports/`
18. `public/`

## Source-of-truth matrix

| Question | Canonical owner | Supporting docs | Must not duplicate |
|---|---|---|---|
| What is Punk? | `README.md` + `docs/product/START-HERE.md` | `PUBLIC-NARRATIVE.md` | Full roadmap, schemas, ADR rationale |
| What are the hard laws? | `docs/product/PUNK-LAWS.md` | ADRs, architecture docs | Phase details, implementation plans |
| What is active, incubating, parked, or retired? | `docs/product/ROADMAP.md` + `docs/product/CRATE-STATUS.md` | `START-HERE.md` | Product hype or future-only operator paths |
| What is the runtime/lifecycle model? | `docs/product/ARCHITECTURE.md` | `PUNK-LAWS.md`, `ROADMAP.md` | Research rationale, phase schedules |
| What does the core own? | `docs/product/ARCHITECTURE.md` | `CRATE-STATUS.md` | Crate-by-crate implementation detail unless needed |
| What crates/folders are active? | `docs/product/CRATE-STATUS.md` | `ROADMAP.md` | Product thesis or long rationale |
| When is research required? | `docs/product/RESEARCH-GATE.md` | `RESEARCH-INTAKE.md` | Full competitor notes or idea backlog |
| How are external ideas classified? | `docs/product/RESEARCH-INTAKE.md` | `knowledge/research/`, `knowledge/ideas/` | Architecture decisions without ADR promotion |
| What did research find? | `knowledge/research/` | `RESEARCH-GATE.md` | Product truth before promotion |
| Where do speculative ideas live? | `knowledge/ideas/` | `RESEARCH-INTAKE.md` | Current operator docs |
| What is local trust telemetry? | `docs/product/TELEMETRY.md` | ADR-0011, architecture, roadmap | Product analytics or remote export claims |
| What are eval semantics? | `docs/product/EVAL-PLANE.md` | `ROADMAP.md`, eval specs | Final acceptance decisions |
| What is proofpack provenance? | `docs/adr/ADR-0012-*` + `docs/product/ARCHITECTURE.md` | `TELEMETRY.md`, `ROADMAP.md` | Full supply-chain compliance claims |
| What is the gate/assessment boundary? | `docs/product/PUNK-LAWS.md` + ADR-0013 | `ARCHITECTURE.md` | Language implying modules/adapters decide |
| What is project memory? | `docs/product/PROJECT-MEMORY.md` | `DOGFOODING.md`, `PUBLIC-NARRATIVE.md` | Hidden runtime memory or giant prompt memory |
| What is public narrative? | `docs/product/PUBLIC-NARRATIVE.md` | `README.md`, `PROJECT-MEMORY.md` | PubPunk automation claims before promotion |
| Why was an architecture decision made? | `docs/adr/` | `knowledge/research/` | Current-state summaries outside the ADR |
| What work is being dogfooded? | `work/goals/`, `work/reports/` | `DOGFOODING.md` | Canonical architecture truth |
| What runtime evidence exists? | `.punk/` | `TELEMETRY.md`, `ARCHITECTURE.md` | Hand-edited product docs |

## Precedence for conflicts

When two docs disagree, resolve in this order:

1. `docs/product/PUNK-LAWS.md`
2. Accepted ADRs in `docs/adr/`
3. Product docs by ownership from the source-of-truth matrix
4. `docs/product/START-HERE.md`
5. `README.md`
6. Accepted research notes in `knowledge/research/`
7. Ideas in `knowledge/ideas/`
8. Work reports in `work/reports/`
9. Public narrative in `public/`

Research and ideas do not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Update rules

### README

Use `README.md` as the public entry.

Allowed:

- short identity
- current active target
- short non-active warning
- links to canonical docs

Avoid:

- schema details
- roadmap phase details
- research rationale
- future module/operator examples

### START-HERE

Use `START-HERE.md` as the internal orientation page.

Allowed:

- reading path
- active operator surface
- short rules for what not to build now
- links to canonical docs

Avoid:

- duplicating full laws
- duplicating full roadmap
- defining schemas
- storing research conclusions directly

### Product docs

Each product doc should own one surface.

If a section explains another surface, it should be short and link to the owner.

### ADRs

ADRs explain accepted decisions and their rationale.

Do not use ADRs as the only place where the current operator path is documented. Product docs should point to ADRs and reflect accepted decisions.

### Research notes

Research notes are advisory by default. They should include adoption maps and eval implications, but they are not implementation truth until promoted.

### Idea backlog

Ideas may be useful, speculative, parked, or anti-patterns. Ideas must not appear in current operator docs unless explicitly promoted.

## Duplication policy

Allowed duplication:

- one-sentence summaries
- short active/not-active lists in entry docs
- links to canonical owners
- status labels repeated for clarity

Not allowed:

- two different definitions of the same lifecycle object
- schemas copied into multiple docs
- roadmap status repeated as if active in future/module docs
- research conclusions presented as accepted architecture without ADR/roadmap promotion
- public claims that exceed canonical docs

## Status vocabulary

Use these terms exactly:

- `active-core` — part of the current stability target; must stay green
- `incubating` — exists and may be tested, but is not default/user-facing
- `parked` — boundary exists; minimal stub/docs only
- `retired` — removed or legacy-only

## Documentation consolidation checklist

Before editing docs, check:

1. Which document owns this truth?
2. Is this active-core, incubating, parked, future, or retired?
3. Does the edit change a law, roadmap phase, storage model, module boundary, adapter boundary, eval policy, project memory, or public CLI contract?
4. If yes, is Research Gate required?
5. Does the edit introduce duplicate schema text?
6. Does it imply a future capability is active?
7. Does it create or remove a source of truth?
8. Are ADR/research/idea/work refs updated?

## Current consolidation gaps

The current docs are directionally coherent, but the following improvements should be made:

1. Add this documentation map.
2. Add a shared glossary for lifecycle, evidence, memory, and status terms.
3. Shorten `README.md` and `START-HERE.md` so they point to owners rather than restating details.
4. Add a docs consistency eval spec.
5. Keep competitor and prior-art findings in `knowledge/research/` and `knowledge/ideas/`, not in operator docs.

