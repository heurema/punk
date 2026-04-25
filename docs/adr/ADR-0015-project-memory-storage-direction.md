# ADR-0015: Project Memory task/work storage direction

Status: Proposed
Date: 2026-04-25

Research refs:

- `knowledge/research/2026-04-25-task-work-storage-before-project-memory.md`

## Context

Punk needs a storage direction for task/work state before implementing Project Memory runtime storage.

Current active behavior is Dogfooding Level 0:

```text
work/STATUS.md -> work/goals/ -> work/reports/
```

Runtime `.punk/` storage, Event Ledger writes, gate decisions, proofpack writing, provider/model/agent adapters, and service-backed boards are intentionally deferred.

The storage model must not create a second tracker, hidden source of truth, or setup requirement for the user.

## Decision

Use a layered storage direction for Project Memory task/work state:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events, when runtime transitions exist
  -> rebuildable SQLite indexes/views for query and inspect UX
  -> optional service-backed adapter/sync after local authority is stable
```

In the current phase:

- `work/STATUS.md` remains the only live work-state source of truth.
- `work/goals/` remain the durable work-intent surface.
- `work/reports/` remain the durable evidence, outcome, and handoff surface.
- Research notes and ADRs record storage rationale and decisions.
- `.punk/` runtime task/work state is not written.

Future JSONL event streams may record runtime transitions and receipts, but they must not be the only home for rich human-authored goals or reports.

Future SQLite/FTS storage may provide query and inspect projections, but it must be rebuildable and non-authoritative.

Future service-backed storage may mirror, sync, or coordinate after authority is stable, but it must not own active-core truth.

## Consequences

Positive:

- preserves one project, one live ledger;
- keeps current work reviewable through Git and plain text;
- avoids committing to a database or service before gate/proof storage exists;
- leaves a clear path to replayable events and fast derived views;
- preserves setup-neutral, ambient operation.

Negative / costs:

- manual Markdown remains less queryable until derived indexes exist;
- `work/STATUS.md` can still conflict under concurrent edits;
- future JSONL replay and SQLite rebuild need explicit validators;
- richer team board UX stays parked.

## Active, deferred, parked

Active now:

- repo-tracked manual Work Ledger;
- reports with evidence and DocImpact;
- research notes and ADRs for storage direction.

Deferred:

- `.punk/events` runtime event stream;
- `.punk/indexes` or `.punk/views` SQLite/FTS projections;
- migration tooling;
- runtime gate/proof/storage acceptance.

Parked:

- service-backed task storage as authority;
- required daemon/server/hook setup;
- remote-first board semantics;
- provider/model/agent adapters as storage owners.

Avoid:

- canonical SQLite task/work database before authority rules are tested;
- JSONL-only human work ledger;
- duplicate selected-work truth in both `work/STATUS.md` and another tracker;
- hidden service state required to know what work is selected.

## Required eval direction

Before implementation, define eval/spec cases for:

- one Level 0 live ledger;
- exactly one ready selected goal;
- repo Markdown authority over derived views;
- malformed/truncated JSONL detection;
- deterministic SQLite rebuild;
- stale/missing index degradation;
- no service-backed authority;
- no `.punk/` task/work runtime writes before a selected implementation goal;
- migration preservation of goal, report, receipt, and decision refs;
- executor claims remaining below proof/gate authority.

## Gate note

This ADR is proposed, not accepted.

Only future `gate` writes final acceptance. Level 0 `done` records manual closure with evidence.
