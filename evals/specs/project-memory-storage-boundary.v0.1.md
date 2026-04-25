# Project Memory storage boundary v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Project Memory task/work storage must satisfy before any `.punk/` runtime task state, event replay storage, SQLite index, service-backed sync, storage schema, or CLI implementation begins.

This is a design/spec artifact only.

It protects the user flow:

```text
current work -> bounded goal -> report/evidence -> review -> next selected work
```

from becoming a hidden tracker, hidden database, or agent-local memory.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate `.punk/` task/work storage.

It does not define a storage schema, CLI command, database migration, service API, gate behavior, proofpack writer, or automation.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and must keep `docs/product/PROJECT-MEMORY.md` and `docs/adr/ADR-0015-project-memory-storage-direction.md` in sync.

## Boundary model

Project Memory task/work storage follows this direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events, when runtime transitions exist
  -> rebuildable SQLite indexes/views for query and inspect UX
  -> optional service-backed adapter/sync after local authority is stable
```

The key rule is:

```text
authority first, views later
```

A faster or prettier view may help inspection, but it must not become the source of truth.

## Level 0 authority: repo Markdown

At Dogfooding Level 0, the authoritative user-visible work state is:

- `work/STATUS.md` for the one live work-state ledger;
- `work/goals/` for durable work intent;
- `work/reports/` for durable outcome, evidence, and handoff artifacts.

This layer answers:

- what is current;
- what is selected next;
- what is blocked;
- what was completed;
- what evidence and reports exist.

Level 0 `done` means manual closure with evidence.

It does not mean future `gate` acceptance.

## Future JSONL runtime-event boundary

A future JSONL event stream may record runtime observations such as transitions, receipts, checks, or storage replay events.

It may be useful for:

- append-only runtime history;
- replay;
- event integrity checks;
- linking receipts and later gate/proof artifacts.

It must not be the only home for:

- rich human-authored goal scope;
- current selected work;
- handoff reports;
- ADR decisions;
- final acceptance.

Malformed or truncated JSONL must be detected before replay is trusted.

If JSONL and repo Markdown disagree about Level 0 selected work, repo Markdown wins until a later accepted runtime-storage goal explicitly changes the authority model.

## Future SQLite derived-index boundary

A future SQLite or FTS store may provide:

- fast search;
- materialized views;
- inspect UX;
- derived status summaries;
- queryable link graphs.

It must be rebuildable from authoritative artifacts and append-only runtime events.

It must not own project truth.

If the index is missing, stale, corrupt, or behind the source artifacts, Punk should report degraded inspect/search behavior rather than change the work state.

A derived SQLite row cannot override `work/STATUS.md`, `work/goals/`, `work/reports/`, ADRs, or future gate decisions.

## Future service-backed boundary

A future service-backed system may later mirror, sync, coordinate, or display task/work state.

It may help with:

- team visibility;
- approvals;
- remote board UX;
- multi-user coordination;
- external integrations.

It must not be active-core authority until a later decision explicitly changes the architecture.

For now, service-backed storage must not be required to know:

- current focus;
- selected next goal;
- completed work;
- gate/proof status;
- source-of-truth project memory.

No hidden remote state may silently decide or hide selected work.

## Authority matrix

| Surface | Current role | Authority now | Future role | Must not do |
|---|---|---:|---|---|
| `work/STATUS.md` | One live ledger | yes | May later be reflected in derived views | Compete with another live ledger |
| `work/goals/` | Work intent | yes | May later link to contracts/events | Become unlinked chat/task scraps |
| `work/reports/` | Outcome and evidence handoff | yes | May later link to receipts/proofs | Become proof or gate by itself |
| JSONL runtime events | Not active for task storage | no | Append-only runtime observations | Own rich human-authored task truth |
| SQLite/FTS | Not active for task storage | no | Rebuildable index/view | Override source artifacts |
| Service-backed storage | Parked | no | Mirror/sync/coordinate | Hide current selected work |
| Executor-local memory | Not project memory | no | Unverified input until captured | Become implementation truth |
| Future `gate` | Not implemented | no | Final closure authority | Be implied by Level 0 `done` |

## Required deterministic eval cases

Before implementation, the eval/spec family must cover at least these cases.

### PM-STORAGE-001: one live Level 0 ledger

Given repo-tracked Level 0 work artifacts, there must be exactly one live status ledger:

```text
work/STATUS.md
```

No other tracker, service file, generated view, or hidden runtime state may claim current selected work.

### PM-STORAGE-002: selected next is exactly one ready goal

`work/STATUS.md.selected_next` must point to one existing `work/goals/*.md` artifact with status `ready`.

### PM-STORAGE-003: no `.punk/` task storage required for Level 0 truth

Manual Level 0 truth must remain inspectable from repo-tracked artifacts even if `.punk/` does not exist.

### PM-STORAGE-004: malformed JSONL is not silently accepted

A future JSONL replay/input path must reject or quarantine malformed lines and report the failure.

It must not silently skip malformed data while claiming full replay success.

### PM-STORAGE-005: truncated JSONL is detected before trusted replay

A future JSONL replay/input path must detect truncated or incomplete event streams before using them as trusted runtime history.

### PM-STORAGE-006: SQLite rebuild is deterministic

A future SQLite/FTS index must be rebuildable from authoritative repo artifacts and any accepted event stream.

Given the same inputs, rebuild output must be stable enough for inspect/eval assertions.

### PM-STORAGE-007: stale or missing SQLite index degrades only views

If a future SQLite/FTS index is missing, stale, corrupt, or older than source artifacts, Punk must report degraded inspect/search behavior.

It must not change selected work or acceptance state.

### PM-STORAGE-008: derived views cannot override repo authority

If a future derived view disagrees with repo Markdown authority, the eval must fail unless the view is rebuilt or clearly marked stale.

### PM-STORAGE-009: service-backed storage is not authority

A future service-backed mirror/sync view must not be required to determine current selected work in active-core.

If service state disagrees with local authority, local authority wins until a later accepted architecture decision says otherwise.

### PM-STORAGE-010: gate/proof terms do not imply implementation

Docs, specs, reports, and views must not describe `gate`, proofpack, `.punk/decisions`, or `.punk/proofs` as active runtime behavior before implementation exists.

### PM-STORAGE-011: executor claims are not proof

A report or executor output may claim checks passed, but storage/index/projection layers must not convert that claim into proof or acceptance.

### PM-STORAGE-012: migrations preserve refs

Any future migration touching task/work storage must preserve at least goal refs, report refs, receipt refs, decision refs, and proof refs when those surfaces exist.

### PM-STORAGE-013: worktree or branch collisions do not create two live ledgers

Parallel worktrees or branches may have divergent edits, but a project checkout must still expose one live `work/STATUS.md` ledger for the current checkout.

Collision handling must be explicit rather than hidden in a derived DB.

### PM-STORAGE-014: speculative knowledge is excluded by default

Future retrieval/index behavior must exclude speculative knowledge from implementation truth by default unless a bounded goal explicitly cites it as advisory input.

## Non-goals

This v0.1 boundary does not define:

- runtime storage implementation;
- `.punk/` task/work directory layout;
- JSONL schema;
- SQLite schema;
- migrations;
- service API;
- storage CLI;
- `punk init`;
- gate implementation;
- proofpack writer;
- provider/model/agent adapters;
- automation;
- a new tracker product.

## Future implementation constraints

Any later implementation step should be explicit about:

- which artifact owns each fact;
- append-only semantics for runtime events;
- malformed/truncated event handling;
- derived index rebuild inputs;
- stale index detection;
- service-backed non-authority semantics;
- migration invariants;
- no hidden selected-work truth;
- how inspect UX distinguishes authority from views.

## Deferred for later specs or goals

Still deferred after this boundary spec:

- missing-validator policy;
- minimal receipt fields;
- semantic assessor command interface;
- real `.punk/` runtime storage;
- real gate runtime;
- real proofpack runtime;
- possible future `punk init`.

## Next bounded step

After this boundary spec, the conservative next move is an advisory Work Ledger Review before selecting runtime storage, missing-validator, receipt, or gate/proof implementation work.
