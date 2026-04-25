---
id: docs_product_project_memory
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-25
review_after: 2026-07-20
canonical_for:
  - project-memory-model
  - repo-tracked-memory-surfaces
  - runtime-derived-memory-boundary
  - knowledge-authority-model
  - project-coherence-boundary
  - documentation-as-project-memory
  - manual-work-ledger-boundary
  - task-work-storage-direction
  - prompt-and-skill-memory-boundary
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/DOC-GOVERNANCE.md
  - docs/product/PUBLIC-NARRATIVE.md
related_adrs:
  - docs/adr/ADR-0003-project-memory-plane.md
  - docs/adr/ADR-0008-knowledge-vault-boundaries.md
  - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
  - docs/adr/ADR-0015-project-memory-storage-direction.md
supersedes: []
superseded_by: null
---
# Project Memory

## Purpose

Single-contract correctness is not enough.

`punk` must also preserve project trajectory across many goals, contracts, reports, and decisions.

## Components

```text
Project Memory Plane
├─ Work Ledger / Contract Tracker
├─ Knowledge Vault
└─ Project Coherence Gate
```

## Link graph

Project memory should preserve explicit links across:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

This keeps memory bounded and inspectable. The graph should be built from canonical artifacts, not from one giant prompt or hidden runtime state.

## Prompt and skill memory boundary

Prompts, skills, playbooks, model-specific settings, and executor-local memories are not project memory by default.

They may help an executor perform work, but they do not own authority and must not become hidden implementation truth.

Executor-local memory can be cited only as an unverified input until a Punk artifact captures it with scope, source, date, and authority.

A useful executor failure should be promoted into one of:

- a contract clause
- a validator
- a receipt or evidence field
- an eval case
- a proof requirement
- a report
- an ADR or knowledge update
- a bounded runner aid with explicit scope and review date

Verified executor output becomes project memory only through artifact links such as:

```text
goal -> contract -> receipt -> eval/assessment -> gate decision -> proofpack -> docs/ADR
```

Project memory is built from linked artifacts, not accumulated prompt text or executor-local state.

## Repo-tracked memory

```text
work/
  STATUS.md
  goals/
  reports/
  views/

knowledge/
  product/
  architecture/
  decisions/
  research/
  experiments/
  ops/
  ideas/
  code/

docs/
  product/
  adr/
  archive/

publishing/
```

## Level 0 manual Work Ledger

Before runtime contract tracking exists, Punk uses a repo-tracked manual Work Ledger under `work/`.

The canonical live state is:

```text
work/STATUS.md
```

This file is the Dogfooding Level 0 live-status surface.

It must make the current focus, selected next goal, blockers, recent completed items, and validation state inspectable without hidden chat context.

At this stage:

- `work/STATUS.md` is human-maintained;
- `work/goals/` hold durable work intent;
- `work/reports/` hold durable outcome and handoff artifacts;
- `.punk/` runtime state is not written yet for this purpose;
- this surface must not become a second tracker product or a hidden backend.

Level 0 `done` means manual closure with evidence.

It does not mean future `gate` acceptance.

## Documentation as project memory

Canonical product and process docs live under `docs/product/`.

Decision history lives under `docs/adr/`.

Historical-only docs live under `docs/archive/` when a current surface is replaced.

Documentation should carry explicit owner, status, authority, and supersession/archive markers as the docs lifecycle matures.

`DocImpact` links meaningful changes to the docs surfaces they must update.

See `docs/product/DOC-GOVERNANCE.md`.

## Runtime/derived memory

```text
.punk/
  project/
  indexes/
  events/
  contracts/
  runs/
  evals/
  decisions/
  proofs/
  views/
```

## Task/work storage direction

Project Memory task/work storage follows this direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events, when runtime transitions exist
  -> rebuildable SQLite indexes/views for query and inspect UX
  -> optional service-backed adapter/sync after local authority is stable
```

Current Level 0 authority remains in repo-tracked Markdown:

- `work/STATUS.md` owns the one live work-state ledger;
- `work/goals/` owns durable work intent;
- `work/reports/` owns durable outcome, evidence, and handoff artifacts;
- `docs/adr/` and `knowledge/research/` own decision rationale and advisory research.

Future JSONL event streams may record runtime transitions and receipts, but they must not be the only home for rich human-authored goals or reports.

Future SQLite/FTS storage may provide query and inspect projections, but it must be rebuildable and non-authoritative. A missing or stale index degrades inspect UX; it does not change project truth.

Future service-backed storage may mirror, sync, or coordinate after local authority is stable, but it must not own active-core truth or hide selected work.

No `.punk/` task/work runtime writes are active until a later selected implementation goal explicitly activates them.

See `docs/adr/ADR-0015-project-memory-storage-direction.md`.

## Non-canonical retrieval memory

Repo-search and code-retrieval systems may keep runtime state such as:

- frecency databases
- query history
- file indexes
- symbol indexes
- vector indexes
- language-server caches

This state is retrieval memory, not project memory.

Retrieval memory may help rank or locate evidence, but it must not become a
source of implementation truth. It should be treated as runtime/derived state
and should be inspectable through receipts or explicit index metadata when it
affects a contract run.

If a retrieval result should become durable project knowledge, it must be
promoted through the normal path:

```text
retrieval receipt
  -> report / research note
  -> ADR or knowledge update
  -> contract refs
  -> gate/proof
```

## Knowledge authority

Every knowledge artifact has:

- `status`
- `authority`

Recommended authorities:

- `canonical`
- `advisory`
- `speculative`

Implementation retrieval excludes speculative knowledge by default.

When retrieval grows beyond manual repo inspection, the artifact contract should make source refs, review windows, supersession, contradiction, and proof/decision links explicit instead of hiding them in chat or runtime state.

## Project coherence

The project-level gate asks:

Are all accepted contracts still moving the project in the intended direction?

It checks:

- accepted contracts since last review
- touched components
- project scope expansion
- missing knowledge updates
- stale docs
- unresolved cleanup obligations
- blocked/escalated work

## Knowledge Vault operating boundary

Knowledge Vault is the repo-tracked knowledge surface of the Project Memory Plane.

It is not:

- a runtime memory engine
- a vector DB source of truth
- hidden agent memory
- a remote/shared commons
- a module-owned decision surface

Repo-tracked truth lives in `work/`, `knowledge/`, `docs/adr/`, and `publishing/`.

`.punk/` may hold derived indexes and views, but derived state is rebuildable and non-authoritative.

Future retrieval must:

- stay advisory-only
- cite repo artifact paths
- exclude speculative knowledge by default
- flag stale or superseded knowledge
- surface contradictions instead of flattening them
- never write final decisions
- never bypass `plot / cut / gate`

Promotion path:

```text
idea/research -> ADR or roadmap decision -> goal/contract -> implementation -> eval result -> proof/knowledge update
```

Promotion is nomination, not move: the source artifact stays intact until a separate decision promotes or supersedes it.

## Research as input to project memory

Research notes live under `knowledge/research/`.

They are advisory by default.

They can support:

- ADRs
- roadmap decisions
- contract constraints
- eval case design
- knowledge updates

Research is not canonical truth until promoted.

## Public memory

Public narrative is part of project memory.

Public-facing artifacts live under `publishing/`.

They are separate from internal `knowledge/`, but can link to canonical knowledge and goals.

The future PubPunk module must use this structure.
