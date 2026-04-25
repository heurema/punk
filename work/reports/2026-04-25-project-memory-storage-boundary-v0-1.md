---
id: report_2026_04_25_project_memory_storage_boundary_v0_1
goal_id: goal_define_project_memory_storage_boundary_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Define Project Memory storage boundary v0.1 as a docs/spec-only artifact before runtime storage implementation.

## Research Gate

Classification: R1
Required: yes
Rationale:
The boundary is a design/spec follow-up grounded in the completed R2 storage research and canonical Project Memory docs. No new Deep Research was needed.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `knowledge/research/2026-04-25-task-work-storage-before-project-memory.md`
- `docs/adr/ADR-0015-project-memory-storage-direction.md`
- `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`

Decision:
Proceed with docs/spec/work-ledger artifacts only.

## Changed files

- `evals/specs/project-memory-storage-boundary.v0.1.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`
- `work/goals/goal_run_fifth_work_ledger_review.md`
- `work/reports/2026-04-25-project-memory-storage-boundary-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `evals/specs/project-memory-storage-boundary.v0.1.md`.
- Linked the boundary spec from `docs/product/PROJECT-MEMORY.md`.
- Closed the selected storage-boundary goal with report/spec refs.
- Added the fifth advisory Work Ledger Review goal as the next ready goal.
- Updated `work/STATUS.md` so the single live ledger selects the review.

## Boundary summary

The user-flow rule is:

```text
authority first, views later
```

Current authority remains:

- `work/STATUS.md` for one live ledger;
- `work/goals/` for durable work intent;
- `work/reports/` for outcome, evidence, and handoff.

Future layers are constrained:

- JSONL may record append-only runtime events, but not rich human-authored task truth.
- SQLite/FTS may provide rebuildable views, but not override source artifacts.
- Service-backed storage may mirror/sync/coordinate later, but not own active-core authority.

## Required eval cases captured

The boundary spec captures deterministic cases for:

- one live Level 0 ledger;
- exactly one ready selected goal;
- no `.punk/` task storage required for Level 0 truth;
- malformed JSONL rejection/quarantine;
- truncated JSONL detection;
- deterministic SQLite rebuild;
- stale/missing SQLite index degradation;
- derived views not overriding repo authority;
- service-backed storage non-authority;
- gate/proof terms not implying implementation;
- executor claims not becoming proof;
- migration ref preservation;
- worktree/branch collisions not creating two live ledgers;
- speculative knowledge excluded by default.

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
  classification: architecture
  reason: "Project Memory storage boundary v0.1 codifies the authority/view split before runtime storage implementation."
  touched_surfaces:
    - evals/specs/project-memory-storage-boundary.v0.1.md
    - docs/product/PROJECT-MEMORY.md
    - work/goals/goal_define_project_memory_storage_boundary_v0_1.md
    - work/goals/goal_run_fifth_work_ledger_review.md
    - work/reports/2026-04-25-project-memory-storage-boundary-v0-1.md
    - work/STATUS.md
  required_updates:
    - docs/product/PROJECT-MEMORY.md
    - evals/specs/project-memory-storage-boundary.v0.1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future implementation must satisfy project-memory-storage-boundary.v0.1 cases before runtime storage activation."
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_run_fifth_work_ledger_review.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_run_fifth_work_ledger_review.md`; goals checked: 49.
- `scripts/check.sh docs-governance --files evals/specs/project-memory-storage-boundary.v0.1.md docs/product/PROJECT-MEMORY.md work/goals/goal_define_project_memory_storage_boundary_v0_1.md work/goals/goal_run_fifth_work_ledger_review.md work/reports/2026-04-25-project-memory-storage-boundary-v0-1.md work/STATUS.md --report work/reports/2026-04-25-project-memory-storage-boundary-v0-1.md` - PASS with 1 warning; changed files: 6; canonical docs checked: 1; reports checked: 1; failures: 0. Warning: `DOC_DUPLICATE_DEFINITION_CANDIDATE` for pre-existing `Project coherence` section in `docs/product/PROJECT-MEMORY.md`.
- `cargo test --workspace` - PASS; 53 Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Run the fifth advisory Work Ledger Review.
- Define missing-validator policy.
- Define minimal receipt fields.
- Define semantic assessor command interface.
- Implement real `.punk/` runtime storage only through a later bounded goal.
- Implement real gate/proofpack runtime only through later bounded goals.
- Revisit possible future `punk init` only when runtime setup is intentionally scoped.
