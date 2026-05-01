---
id: report_2026_05_01_roadmap_status_reconciliation_after_greenfield_init_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_reconcile_roadmap_after_greenfield_init_v0_1.md
---

# Roadmap/Status Reconciliation After Greenfield Init v0.1

## Summary

- Reconciled the remaining canonical-doc drift after compact greenfield init became active.
- Updated `docs/product/NORTH-STAR.md` so its current executable CLI list includes `punk init <project-id>`.
- Made no code changes and added no product behavior.

## Files Inspected

- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `work/STATUS.md`
- `work/goals/goal_reconcile_roadmap_after_greenfield_init_v0_1.md`
- `work/reports/2026-05-01-greenfield-init-checkpoint-status-v0-1.md`
- `work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md`

## Active CLI/Status Corrections

The reconciled active CLI surface is:

```text
punk init <project-id>
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

`docs/product/NORTH-STAR.md` previously omitted `punk init <project-id>` from this current executable CLI list. It now includes the command with a narrow greenfield-only caveat.

## Greenfield Init Scope

`punk init <project-id>` remains limited to minimal greenfield project-memory setup:

- compact `.punk/memory/` layout;
- repo-tracked durable project-memory seed;
- `.punk/README.md` and `.punk/project.toml` marker/setup files;
- `project_id`, `entry_mode = greenfield`, compact memory metadata, and inactive runtime metadata;
- no root-level `work/`, `knowledge/`, `docs/adr/`, or `publishing/` for ordinary user projects.

## Brownfield/Grayfield Deferred Boundary

Brownfield init and grayfield reconciliation remain deferred.

No project archaeology, reconstruction pack, AI summary, generated doc, repo scan, or advisory memory promotion behavior is active.

## Dogfooding Root Layout vs User Compact Layout

The Punk repository may keep its root dogfooding layout:

```text
work/
knowledge/
docs/adr/
publishing/
```

User projects initialized by `punk init <project-id>` use the compact layout:

```text
.punk/memory/
```

This distinction is already clear in `docs/product/START-HERE.md`, `docs/product/PROJECT-MEMORY.md`, `docs/product/ROADMAP.md`, and `README.md`.

## Runtime `.punk` Boundary

Runtime and derived `.punk` storage remains inactive:

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

The reconciliation did not activate contracts, contract generation, run receipts, gate/proof runtime, proofpacks, acceptance claims, Writer, agents/adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

## Docs Changed

- `docs/product/NORTH-STAR.md`
  - updated `updated_at`;
  - added `punk init <project-id>` to the current executable CLI list;
  - added a narrow scope caveat for greenfield compact `.punk/memory/` init and explicit inactive/deferred boundaries.

## Docs Left Unchanged and Why

- `README.md` already lists `punk init <project-id>` as current behavior and states brownfield/grayfield/runtime/Writer boundaries.
- `docs/product/START-HERE.md` already lists the active CLI surface and compact greenfield init boundary.
- `docs/product/ROADMAP.md` already records Phase 0 active target and exit criteria for compact `.punk/memory/` init.
- `docs/product/CRATE-STATUS.md` already records `punk-cli`, `punk-project`, and current CLI status accurately.
- `docs/product/PROJECT-MEMORY.md` already distinguishes Punk dogfooding root layout from user compact `.punk/memory/` layout.
- `docs/product/DOCUMENTATION-MAP.md` did not need ownership changes.
- `docs/product/GLOSSARY.md` did not need new terms for this reconciliation.

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files docs/product/NORTH-STAR.md work/STATUS.md work/goals/goal_reconcile_roadmap_after_greenfield_init_v0_1.md work/goals/goal_pause_after_greenfield_init_checkpoint_v0_1.md work/reports/2026-05-01-roadmap-status-reconciliation-after-greenfield-init-v0-1.md --report work/reports/2026-05-01-roadmap-status-reconciliation-after-greenfield-init-v0-1.md` - PASS with 0 failures and 0 warnings.

## Drift Observed

- Fixed: `docs/product/NORTH-STAR.md` current executable CLI list omitted active `punk init <project-id>`.
- No implementation drift observed.

## Remaining Warnings

- None in this reconciliation scope.

## Next Selected Goal

`work/goals/goal_pause_after_greenfield_init_checkpoint_v0_1.md`

Reason: docs/status are now honest and no immediate product work is required.

## Doc impact

```yaml
  classification: docs-only
  reason: "Reconciles current CLI status after verified compact greenfield init without changing product behavior."
```
