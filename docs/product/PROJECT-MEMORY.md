---
id: docs_product_project_memory
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-05-01
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
  - contract-context-pack-memory-boundary
  - knowledge-impact-report-boundary
  - replayable-project-memory-direction
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/DOC-GOVERNANCE.md
  - docs/product/PUBLIC-NARRATIVE.md
related_adrs:
  - docs/adr/ADR-0003-project-memory-plane.md
  - docs/adr/ADR-0008-knowledge-vault-boundaries.md
  - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
  - docs/adr/ADR-0015-project-memory-storage-direction.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
related_evals:
  - evals/specs/project-memory-storage-boundary.v0.1.md
  - evals/specs/context-pack-boundary.v0.1.md
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

## Replayable project memory direction

Project memory should not only preserve what happened. It should preserve enough linked intent, contracts, evidence, decisions, examples, invariants, and proof refs to help future work replay intent and draft migration/reimplementation contracts.

This is a directional memory quality, not a new runtime feature.

Greenfield Punk projects record replay memory from day zero. Brownfield projects reconstruct replay candidates from existing artifacts. Grayfield projects reconcile partial recorded memory with partial reconstruction.

Project memory remains repo-tracked and authority-tagged. Derived summaries, future conformance packs, migration packs, or regenerative spec packs must not become truth without review, gate decisions, and proof.

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

## Contract Context Pack memory boundary

A Contract Context Pack is a contract-linked evidence-selection artifact
prepared during `plot`.

It is project-memory-relevant because it explains why specific context was
selected, excluded, marked stale, or treated as unknown for a contract.

It is not project truth by itself.

A context pack may become part of the project-memory link graph only through
explicit artifact refs, for example:

```text
goal -> contract -> context pack -> run/report -> eval -> gate decision -> proof
```

The pack should preserve:

- selected source refs;
- short rationales;
- bidirectional clause/support mappings;
- authority and status metadata;
- explicit exclusions;
- stale or superseded flags;
- contradiction sets;
- unknowns and assumptions;
- retrieval receipt refs when retrieval was used.

The pack must not preserve hidden prompt state as truth. If a prompt, runner
aid, or executor-local memory should become durable project memory, it must be
captured through the normal artifact path with scope, source, date, authority,
review, and proof/gate links where relevant.

Current scope is side-effect-free model/validation and documentation only.
Runtime context-pack storage, derived views, retrieval integration, compression,
and executor brief generation remain deferred or parked.

## Repo-tracked Memory Layouts

Punk separates durable project memory from runtime or derived state.

Durable project memory must be repo-tracked, but it does not always need to live in top-level project directories.

### Punk Dogfooding Layout

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

This root layout is allowed for the Punk repository itself because this repository dogfoods Punk as its own work ledger and product memory surface.

### User Project Default Layout

```text
.punk/
  README.md
  project.toml
  memory/
    STATUS.md
    goals/
    reports/
    knowledge/
      ideas/
      research/
    adr/
```

For user projects, `punk init <project-id>` uses the compact `.punk/memory/` layout by default.

`punk init <project-id> --mode brownfield` uses the same compact memory root and adds only an advisory `.punk/memory/reconstruction/` workspace for future reviewed reconstruction.

`.punk/memory/` is repo-tracked durable project memory.

`.punk/runtime/`, `.punk/cache/`, `.punk/events/`, `.punk/runs/`, `.punk/evals/`, `.punk/contracts/`, `.punk/decisions/`, `.punk/proofs/`, `.punk/indexes/`, and `.punk/views/` remain inactive future runtime or derived state unless a later bounded goal promotes them.

## Level 0 manual Work Ledger

Before runtime contract tracking exists, the Punk repository uses a repo-tracked manual Work Ledger under `work/`, while user projects initialized by `punk init <project-id>` use `.punk/memory/`.

The canonical live state for the Punk repository dogfooding layout is:

```text
work/STATUS.md
```

This file is the Dogfooding Level 0 live-status surface.

For user projects initialized with the compact layout, the equivalent live-status surface is:

```text
.punk/memory/STATUS.md
```

It must make the current focus, selected next goal, blockers, recent completed items, and validation state inspectable without hidden chat context.

At this stage:

- the live status file is human-maintained;
- goals hold durable work intent;
- reports hold durable outcome and handoff artifacts;
- `punk init <project-id>` creates the initial greenfield Level 0 compact scaffold under `.punk/memory/`;
- `punk init <project-id> --mode brownfield` creates a brownfield Level 0 advisory reconstruction scaffold under `.punk/memory/`;
- the greenfield scaffold records `project_id` and `entry_mode = greenfield`;
- the brownfield scaffold records `project_id`, `entry_mode = brownfield`, `reconstruction_status = not_started`, and `authority = advisory_candidates_only`;
- `.punk/README.md` and `.punk/project.toml` may mark the project root and setup metadata;
- `.punk/` runtime stores are not written yet for this purpose;
- brownfield init does not scan the repository, reconstruct project knowledge, generate summaries, generate contracts/specs, accept claims, or prove anything;
- root-level `work/`, `knowledge/`, `docs/adr/`, and `publishing/` are not created by default for user projects;
- this surface must not become a second tracker product or a hidden backend.

The current init CLI surface is limited to manual greenfield memory and brownfield entry scaffolding.
It does not create brownfield reconstruction, grayfield reconciliation, runtime project storage, flow events, contracts, receipts, gate artifacts, proofpacks, or acceptance claims.

Level 0 `done` means manual closure with evidence.

It does not mean future `gate` acceptance.

## Documentation as project memory

Canonical product and process docs live under `docs/product/`.

In the Punk repository dogfooding layout, decision history lives under `docs/adr/`.

In the compact user-project layout, starter ADR notes live under `.punk/memory/adr/`.

Historical-only docs live under `docs/archive/` when a current surface is replaced.

Documentation should carry explicit owner, status, authority, and supersession/archive markers as the docs lifecycle matures.

`DocImpact` links meaningful changes to the docs surfaces they must update.

See `docs/product/DOC-GOVERNANCE.md`.

## Knowledge Impact Report boundary

At Dogfooding Level 0, a Knowledge Impact section in `work/reports/` is the manual way to record project-memory effects before runtime derived views exist.

It complements `DocImpact`: `DocImpact` identifies documentation surfaces that must be updated; Knowledge impact records wider project-memory effects such as stale or suspect docs, affected ADRs/evals/goals/contracts, scope label changes, public narrative impact, derived views to rebuild later, follow-up goals, unknowns, and contradictions.

The section is advisory evidence. It does not write final decisions, replace gate/proof, automatically rewrite canonical truth, or implement Knowledge Vault.

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

Boundary v0.1 is defined in `evals/specs/project-memory-storage-boundary.v0.1.md`. That spec is advisory/design only; it protects the authority/view split before runtime storage implementation.

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

At review time, the project-level gate asks:

Are all accepted contracts still moving the project in the intended direction?

It checks:

- accepted contracts since last review
- touched components
- project scope expansion
- missing knowledge updates
- stale docs
- unresolved cleanup obligations
- blocked/escalated work

## Knowledge Vault direction

Knowledge Vault is not RAG.

Repo artifacts remain the source inputs for project memory. Derived claims, graphs, compiled wiki pages, indexes, embeddings, and views are rebuildable and non-authoritative unless a later promoted artifact explicitly changes their status.

Context packs are advisory task-specific selections. They can explain which sources support a contract, which sources were excluded, and which stale/conflict/unknown signals matter for that task. They are not project truth by themselves.

Canonical truth changes only through reviewed/promoted artifacts such as laws, product docs, ADRs, roadmap decisions, contracts, evals, reports, gate decisions, proofs, or public narrative receipts.

Direction note: `knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md`.

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
