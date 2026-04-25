---
id: report_2026_04_25_task_work_storage_research
goal_id: goal_research_task_storage_before_project_memory
actor: vitaly
created_at: 2026-04-25
kind: research-handoff
---

## Goal

Research task/work storage options before Project Memory implementation, compare repo Markdown, JSONL, SQLite mirror, and service-backed storage, and record a storage direction without changing runtime scope.

## Research Gate

Classification: R2
Required: yes
Rationale:
Task/work storage affects Project Memory, work-ledger authority, future `.punk/` runtime layout, and possible user setup requirements. Prior-art comparison was required before implementation.

Research refs:

- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/DOGFOODING.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0003-project-memory-plane.md`
- `docs/adr/ADR-0009-contract-tracker-core-primitives.md`
- `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`
- `work/STATUS.md`
- `work/goals/goal_research_task_storage_before_project_memory.md`
- `knowledge/research/2026-04-19-contract-tracker-prior-art.md`
- `knowledge/research/2026-04-19-knowledge-vault-prior-art-summary.md`
- Beads README/docs and local `bd` CLI observation, inspected 2026-04-25
- JSON Lines, SQLite WAL, SQLite locking, and SQLite FTS5 docs, fetched 2026-04-25

Decision:
Proceed with research/docs/work-ledger artifacts only. No implementation or runtime storage work.

## Changed files

- `knowledge/research/2026-04-25-task-work-storage-before-project-memory.md`
- `docs/adr/ADR-0015-project-memory-storage-direction.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/goals/goal_research_task_storage_before_project_memory.md`
- `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`
- `work/reports/2026-04-25-task-work-storage-research.md`
- `work/STATUS.md`

## What changed

- Added an R2 research note comparing repo Markdown, JSONL, SQLite mirror/index, and service-backed storage.
- Extracted Beads/minibeads-adjacent failure modes: hidden tracker state, DB/JSONL drift, WAL/sidecar/lock burden, malformed/truncated JSONL, migrations, concurrent writers, hooks/setup effects, redirects/shared DBs, and service/daemon health.
- Added ADR-0015 as the proposed Project Memory task/work storage direction.
- Updated `docs/product/PROJECT-MEMORY.md` with the layered storage direction.
- Closed the selected research goal and selected a docs/spec-only follow-up goal for Project Memory storage boundary v0.1.

## Storage direction

Use a layered direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events, when runtime transitions exist
  -> rebuildable SQLite indexes/views for query and inspect UX
  -> optional service-backed adapter/sync after local authority is stable
```

Current authority remains:

- `work/STATUS.md` for the one live work ledger;
- `work/goals/` for durable work intent;
- `work/reports/` for durable outcomes, evidence, and handoff;
- `docs/adr/` and `knowledge/research/` for decision rationale and advisory evidence.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk/` runtime state written;
- no runtime event ledger added;
- no CLI command added;
- no `punk init` command added;
- no gate implementation added;
- no proofpack writer added;
- no provider/model/agent adapter added;
- no service-backed storage added;
- no automation added;
- no second tracker added.

## Doc impact

```yaml
doc_impact:
  classification: research-promotion
  reason: "R2 storage research produced a proposed ADR and updated the canonical Project Memory storage direction without implementation."
  touched_surfaces:
    - knowledge/research/2026-04-25-task-work-storage-before-project-memory.md
    - docs/adr/ADR-0015-project-memory-storage-direction.md
    - docs/product/PROJECT-MEMORY.md
    - work/goals/goal_research_task_storage_before_project_memory.md
    - work/goals/goal_define_project_memory_storage_boundary_v0_1.md
    - work/reports/2026-04-25-task-work-storage-research.md
    - work/STATUS.md
  required_updates:
    - docs/product/PROJECT-MEMORY.md
    - docs/adr/ADR-0015-project-memory-storage-direction.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Define Project Memory storage boundary v0.1 eval/spec before implementation."
    - "Cover malformed/truncated JSONL, deterministic SQLite rebuild, stale/missing index behavior, one live ledger, and service-backed non-authority."
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`; goals checked: 48.
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-25-task-work-storage-before-project-memory.md docs/adr/ADR-0015-project-memory-storage-direction.md docs/product/PROJECT-MEMORY.md work/goals/goal_research_task_storage_before_project_memory.md work/goals/goal_define_project_memory_storage_boundary_v0_1.md work/reports/2026-04-25-task-work-storage-research.md work/STATUS.md --report work/reports/2026-04-25-task-work-storage-research.md` - PASS with 1 warning; changed files: 7; canonical docs checked: 1; reports checked: 1; failures: 0. Warning: `DOC_DUPLICATE_DEFINITION_CANDIDATE` for pre-existing `Project coherence` section in `docs/product/PROJECT-MEMORY.md`.
- `cargo test --workspace` - PASS; 53 Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Define Project Memory storage boundary v0.1 as the next selected docs/spec-only goal.
- Define missing-validator policy.
- Define minimal receipt fields.
- Define semantic assessor command interface.
- Implement real gate/proofpack runtime only through later bounded goals.
- Revisit real `.punk/` runtime storage after boundary/eval coverage exists.
- Revisit possible future `punk init` only when runtime setup is intentionally scoped.
