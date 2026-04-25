---
kind: research-note
status: active
authority: advisory
created: 2026-04-25
related_goal: work/goals/goal_research_task_storage_before_project_memory.md
related_decision: docs/adr/ADR-0015-project-memory-storage-direction.md
---

# Task/work storage before Project Memory implementation

## Question

What storage direction should Punk use for task/work state before implementing Project Memory runtime storage, without creating a second tracker or weakening the active-core boundary?

## Decision context

Punk currently uses Dogfooding Level 0 manual work tracking:

```text
work/STATUS.md -> work/goals/ -> work/reports/
```

That surface is intentionally repo-tracked and human-readable. Runtime storage under `.punk/`, the Event Ledger, gate decisions, proofpack writing, and richer Project Memory indexes remain deferred.

The storage decision must support future Project Memory while preserving:

- one project, one live ledger;
- no hidden source of truth;
- gate-only final acceptance;
- executor claims as evidence inputs, not proof;
- setup-neutral operation for users.

## Sources reviewed

| Source | Tier | URL / ref | Why included | Key limitation |
|---|---|---|---|---|
| `docs/product/PROJECT-MEMORY.md` | A | repo-local | Canonical Project Memory and manual Work Ledger boundary | Current storage section is intentionally high-level |
| `docs/product/RESEARCH-GATE.md` | A | repo-local | Required R2 process for storage-affecting decisions | Process policy, not a storage design |
| `docs/product/DOGFOODING.md` | A | repo-local | Current Level 0 selected-goal and report discipline | Manual loop only |
| `docs/product/CONTRACT-TRACKER.md` | A | repo-local | Work Ledger / Contract Tracker concept and gate authority | Does not finalize storage |
| `docs/adr/ADR-0003-project-memory-plane.md` | A | repo-local | Existing Project Memory Plane decision | Predates the current manual ledger experience |
| `docs/adr/ADR-0009-contract-tracker-core-primitives.md` | A | repo-local | Defines Contract Tracker as Work Ledger primitives | Leaves `Cut` persistence open |
| `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md` | A | repo-local | Keeps executor outputs below validation authority | Does not choose storage |
| `work/STATUS.md` and recent `work/reports/**` | A | repo-local | Evidence from the actual selected goal -> report -> select next loop | Manual evidence, not runtime proof |
| Beads README and docs | B | https://github.com/steveyegge/beads | Prior art for local/distributed issue graph, Dolt storage, JSONL export, hooks, redirects, server mode | Different product goal: issue tracker for agents |
| Beads CLI observation | B | `bd --version`, `bd --help`, inspected 2026-04-25 | Confirms local command surface, issue graph, JSONL export/import, Dolt/version-control, hooks, daemon/server concepts | Tool output can drift by installed version |
| Local Beads installations | B | inspected 2026-04-25 | Shows real `.beads` shape: DB, JSONL, logs, locks, daemon metadata, sidecars, hooks | Local field evidence only; paths intentionally omitted |
| JSON Lines | A | https://jsonlines.org/ | Format baseline for append-only newline-delimited events | Format spec only, not authority semantics |
| SQLite WAL docs | A | https://www.sqlite.org/wal.html | Source for WAL/sidecar and checkpoint tradeoffs | SQLite docs, not Punk policy |
| SQLite locking docs | A | https://www.sqlite.org/lockingv3.html | Source for file locking/concurrency constraints | SQLite docs, not work-ledger semantics |
| SQLite FTS5 docs | A | https://www.sqlite.org/fts5.html | Source for future search/index projection option | Index feature only |

No local primary `minibeads` source was found through `loci config search minibeads` or local filename search on 2026-04-25. This note treats Beads, its documented legacy SQLite/JSONL history, and Punk's current manual ledger as the available Beads/minibeads-adjacent evidence. If a canonical minibeads artifact appears, rerun this R2 comparison.

## Options compared

| Option | Best use | Strengths | Failure modes | Fit now |
|---|---|---|---|---|
| Repo Markdown | Human-authored goals, reports, ADRs, status | Git reviewable, diffable, readable, already used, no hidden daemon/account/setup | Weak query/search, merge conflicts on one live file, manual discipline required | Adopt as current authority |
| JSONL event stream | Runtime transitions, receipts, replayable append-only facts | Append-only, portable, easy to export/import, can be line-validated | Malformed/truncated lines, hard manual editing, poor rich-text authoring, drift if duplicated with docs | Defer until runtime transitions exist; use below Markdown authority |
| SQLite mirror/index | Query, FTS, materialized views, status projection | Fast, structured, rebuildable index, good for inspect UX | Binary conflicts if canonical, WAL/sidecar/lock complexity, stale indexes, migration data loss | Use only as derived rebuildable state later |
| Service-backed storage | Team board, remote sync, multi-user policy, web UI | Central coordination, approvals, locks, external integrations | Hidden remote truth, account/network dependency, sync conflicts, productization pressure | Park as adapter/sync surface, not active-core authority |

## Beads and adjacent failure modes

Useful patterns from Beads:

- hash-like IDs reduce branch collision risk;
- dependency graphs make blockers explicit;
- JSONL export/import is useful for portability and repair;
- server mode can handle multi-writer cases when the system chooses to own a database service;
- read-only/sandbox flags are useful for worker/executor containment;
- `doctor`/repair commands are a sign that storage needs explicit health checks.

Failure modes Punk should avoid or test before adopting similar mechanics:

- hidden `.beads` style state can become the real tracker if repo artifacts are not clearly authoritative;
- DB plus JSONL plus Markdown can drift unless one source owns each fact;
- WAL, journal, lock, daemon PID, server logs, and sidecar files add setup and recovery burden;
- concurrent writers require a lock/server/conflict policy, not just a local file format;
- malformed or truncated JSONL must be detected and repairable;
- schema migrations can clobber fields unless migration invariants are tested;
- git hooks and setup commands can affect the user's repo workflow, which conflicts with Punk's ambient/setup-neutral direction;
- shared DB redirects are operationally useful but can hide central state outside the current project;
- auto-sync and remote-backed workflows can make push/landing behavior part of the tool contract too early;
- compaction or memory decay can drop useful context unless reports/proofs preserve the evidence path;
- service or daemon health becomes a user-visible dependency if the project truth depends on it.

## Recommendation

Adopt a layered direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events, when runtime transitions exist
  -> rebuildable SQLite indexes/views for query and inspect UX
  -> optional service-backed adapter/sync after local authority is stable
```

Adopt now:

- `work/STATUS.md` remains the only live work-state source of truth.
- `work/goals/` remain durable human-authored work intent.
- `work/reports/` remain durable outcome, evidence, and handoff artifacts.
- ADRs and research notes record storage decisions and rationale.

Defer:

- `.punk/events` as a runtime transition/event stream;
- `.punk/indexes` or `.punk/views` as derived SQLite/FTS projections;
- migration tooling for derived indexes;
- runtime gate/proof/storage acceptance.

Park:

- service-backed task storage as active-core authority;
- remote-first boards;
- required daemon/server/hook setup;
- provider/model/agent adapters as storage owners.

Avoid:

- pure SQLite as canonical task/work storage;
- pure JSONL as the sole place for rich goals/reports;
- a second tracker parallel to `work/STATUS.md`;
- hidden service state that must be consulted to know the selected next goal.

## Storage boundary

For the next Project Memory storage design:

- every durable task/work fact must have one authority path;
- Markdown can own intent, scope, reports, decisions, and narrative evidence;
- JSONL can own append-only runtime observations once those observations exist;
- SQLite can own only derived projections and must be rebuildable;
- a missing or stale SQLite index must degrade inspect UX, not change truth;
- service-backed views may mirror or coordinate later, but cannot own final truth in active-core;
- `.punk/` writes require a later selected implementation goal and explicit evals.

## Required eval cases

Before implementation, add deterministic eval/spec coverage for:

1. `work/STATUS.md` is the only Level 0 live ledger.
2. `selected_next` points to exactly one ready goal.
3. No `.punk/` runtime task state is required for manual Level 0 truth.
4. JSONL event input with a malformed line is rejected or quarantined, not silently accepted.
5. A truncated JSONL event stream is detected before replay.
6. SQLite index rebuild from authoritative sources is deterministic.
7. A stale or missing SQLite index is reported as degraded derived state.
8. SQLite-derived views cannot override repo Markdown authority.
9. Service-backed storage cannot decide or hide selected work.
10. Runtime gate/proof terms cannot imply implementation before a selected runtime goal.
11. Executor claims in reports remain evidence inputs, not proof.
12. Future migrations preserve minimal receipt, goal, report, and decision refs.
13. Worktree or branch collision cases preserve stable IDs and do not create two live ledgers.
14. Speculative knowledge is excluded from implementation retrieval by default.

## Open follow-ups

- Define Project Memory storage boundary v0.1 as a spec/eval artifact.
- Define missing-validator policy.
- Define minimal receipt fields.
- Define semantic assessor command interface.
- Implement real gate/proofpack runtime only after the storage boundary is explicit.
- Revisit `punk init` after runtime storage setup is intentionally scoped.
