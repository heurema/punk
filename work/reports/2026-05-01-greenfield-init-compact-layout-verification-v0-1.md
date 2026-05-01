---
id: report_2026_05_01_greenfield_init_compact_layout_verification_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md
---

# Greenfield Init Compact Layout Verification v0.1

## Summary

- Verified `punk init <project-id>` as the current greenfield compact `.punk/memory/` user-project milestone.
- Found no implementation drift and made no code changes.
- Confirmed docs/status are honest enough; roadmap reconciliation is not needed now.

## Files Inspected

- `work/STATUS.md`
- `work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md`
- `work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md`
- `crates/punk-cli/src/main.rs`
- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `docs/product/START-HERE.md`
- `.gitignore`

## Fresh Temp Init Results

- `~/.local/bin/punk-dev init demo-project` in a fresh temporary directory exited `0`.
- Output reported `schema_version: project-init-greenfield.v0.1`.
- Output reported `entry_mode: greenfield`.
- Output reported `runtime_persistence: inactive`.
- Output reported `result: initialized`.

## Created Files/Directories

Created files:

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

Created directories:

```text
.punk/
.punk/memory/
.punk/memory/goals/
.punk/memory/reports/
.punk/memory/knowledge/
.punk/memory/knowledge/ideas/
.punk/memory/knowledge/research/
.punk/memory/adr/
```

## Absent Root-Level Punk Dirs

Verified absent:

```text
work/
knowledge/
docs/
docs/adr/
publishing/
```

## Absent Runtime Dirs

Verified absent:

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

## Git Visibility Check

After `git init`, `git status --short --untracked-files=all` showed all generated `.punk/README.md`, `.punk/project.toml`, and `.punk/memory/**` files as untracked and therefore trackable.

The repo `.gitignore` does not contain a blanket `.punk/`, `.punk/*`, or `.punk/**` ignore. It ignores only future runtime/derived `.punk/` directories.

## Project Id Validation

- `DemoProject` exited `2` with `invalid project id`.
- `demo_project` exited `2` with `invalid project id`.
- Empty project id exited `2` with `project id must not be empty`.

## Existing/Overwrite Behavior

- Running `punk init demo-project` a second time on matching generated files exited `0` with `result: already_initialized` and all artifacts `already_exists`.
- After modifying `.punk/memory/STATUS.md`, running init again exited `1` with `result: blocked`, reported `.punk/memory/STATUS.md` as `conflict`, and preserved the existing file content.
- This confirms safe idempotency for matching generated files and fail-closed no-overwrite behavior for changed files.

## No-Network/No-Scan/No-Runtime Confirmation

Code inspection found the init path is table-driven over explicit scaffold entries. It uses explicit filesystem metadata checks, `create_dir`, and `OpenOptions::create_new`; it does not use directory traversal, repository scanning, process execution, or networking APIs.

The CLI output and smoke eval also state:

- no brownfield reconstruction;
- no grayfield reconciliation;
- no repo scanning;
- no AI summaries or generated truth;
- no network behavior;
- no contracts, receipts, gate artifacts, proofpacks, or acceptance claims;
- `.punk/runtime` project storage inactive.

## Smoke Eval Result

`~/.local/bin/punk-dev eval run smoke` passed.

The smoke output includes compact init coverage:

```text
greenfield Level 0 scaffold created compact .punk/memory files with project_id, entry_mode, and .punk marker files while leaving root-level Punk memory dirs, brownfield/grayfield, and .punk runtime stores absent
```

## Drift Observed

- No implementation drift observed.
- No docs/status drift requiring roadmap reconciliation observed.
- One behavior note: a second run on unchanged generated files is safe idempotent (`already_initialized`) rather than a hard failure; modified existing files still fail closed and are not overwritten.

## Remaining Warnings

- None found in this verification scope.

## Recommendation

Continue with Option A: `work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md`.

Reason: verification is clean and docs already state that minimal greenfield init is active, so roadmap/status reconciliation is not needed before a milestone status summary.

## Next Selected Goal

`work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md`

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- Manual temp-root `~/.local/bin/punk-dev init demo-project` - PASS.
- Manual project id validation for `DemoProject`, `demo_project`, and empty project id - PASS.
- Manual second-run/idempotency and changed-file conflict checks - PASS.
- Code inspection for no network, no repo scan, no process execution in init path - PASS.
- `.gitignore` inspection for no blanket `.punk/` ignore - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md --report work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md` - PASS with 0 failures and 0 warnings.

## Doc impact

```yaml
  classification: docs-only
  reason: "Records verification evidence for the compact greenfield init milestone without changing product behavior."
```
