---
id: report_2026_05_01_greenfield_init_checkpoint_status_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md
---

# Greenfield Init Checkpoint Status v0.1

## Summary

- Published the minimal greenfield `punk init <project-id>` checkpoint as an active product milestone.
- Made no code changes and added no product behavior.
- Confirmed one remaining docs drift: `docs/product/NORTH-STAR.md` still omits `punk init <project-id>` from its current executable CLI list.

## What Is Now Active

- `punk init <project-id>` is active as a greenfield Dogfooding Level 0 compact manual project-memory scaffold.
- It writes tracked durable user-project memory under `.punk/memory/`.
- It writes only `.punk/README.md`, `.punk/project.toml`, and `.punk/memory/**` marker/memory files.
- It records `project_id`, `entry_mode = greenfield`, compact memory metadata, and inactive runtime metadata.

## What Remains Inactive/Deferred

- Brownfield init.
- Grayfield reconciliation.
- Project archaeology or reconstruction packs.
- Replayable Project Memory runtime.
- Runtime `.punk/` storage.
- Contracts, contract generation, run receipts, gate/proof runtime, proofpacks, and acceptance claims.
- Writer, agents, adapters, provider execution, remote analysis, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, and spec-as-source behavior.

## Verified Init Behavior

Verification evidence is recorded in `work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md`.

Verified created files:

```text
.punk/README.md
.punk/project.toml
.punk/memory/STATUS.md
.punk/memory/goals/goal_initial_project_setup.md
.punk/memory/reports/README.md
.punk/memory/knowledge/ideas/README.md
.punk/memory/knowledge/research/README.md
.punk/memory/adr/README.md
```

Verified project id validation:

- lowercase slug accepted;
- uppercase id rejected;
- underscore id rejected;
- empty id rejected.

Verified existing-file behavior:

- matching second init is safe idempotent and reports `already_initialized`;
- modified existing memory blocks init and preserves the existing file.

## Compact Layout Explanation

The user-project default keeps Punk artifacts contained under `.punk/` while preserving a tracked durable memory boundary:

```text
.punk/memory/   tracked durable project memory
.punk/runtime/  inactive future runtime/evidence state
.punk/cache/    inactive future derived/cache state
```

The Punk repository may keep its root dogfooding layout under `work/`, `knowledge/`, `docs/adr/`, and `publishing/`, but that is not the default user-project init layout.

## Git Visibility / Root-Clutter Result

Fresh temp init plus `git init` showed `.punk/README.md`, `.punk/project.toml`, and `.punk/memory/**` as visible to git.

Verified absent root-level Punk directories:

```text
work/
knowledge/
docs/
docs/adr/
publishing/
```

Verified absent runtime/derived directories:

```text
.punk/events/
.punk/contracts/
.punk/runs/
.punk/evals/
.punk/decisions/
.punk/proofs/
.punk/views/
.punk/indexes/
.punk/runtime/
.punk/cache/
```

The tracked `.gitignore` does not blanket-ignore `.punk/`.

## Brownfield/Grayfield Status

Brownfield and grayfield remain deferred.

Greenfield can record project memory from day zero. Brownfield can only reconstruct candidate memory until reviewed and promoted. Grayfield needs reconciliation between existing Punk memory and reconstructed candidate memory.

None of those reconstruction or reconciliation paths are active behavior.

## Replayability Status

Replayability remains an advisory direction, not runtime behavior.

The active checkpoint supports replayability only by making greenfield project memory inspectable from day zero. It does not activate Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, code generation, migration automation, or spec-as-source behavior.

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md work/goals/goal_reconcile_roadmap_after_greenfield_init_v0_1.md work/reports/2026-05-01-greenfield-init-checkpoint-status-v0-1.md --report work/reports/2026-05-01-greenfield-init-checkpoint-status-v0-1.md` - PASS with 0 failures and 0 warnings.

## Remaining Risks

- `docs/product/NORTH-STAR.md` still omits `punk init <project-id>` from its current executable CLI list. This is a documentation-status drift, not an implementation drift.
- User-facing init docs may still benefit from later polish, but the canonical active/inactive boundary is already clear in `README.md`, `docs/product/START-HERE.md`, `docs/product/ROADMAP.md`, `docs/product/CRATE-STATUS.md`, and `docs/product/PROJECT-MEMORY.md`.

## Recommended Next Goal

`work/goals/goal_reconcile_roadmap_after_greenfield_init_v0_1.md`

Reason: reconcile the remaining canonical-doc status drift before selecting new product behavior or runtime work.

## Doc impact

```yaml
  classification: docs-only
  reason: "Records the verified greenfield init checkpoint and routes a remaining CLI-surface docs drift without changing product behavior."
```
