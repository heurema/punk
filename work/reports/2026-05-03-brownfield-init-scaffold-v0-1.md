---
id: report_2026_05_03_brownfield_init_scaffold_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_add_brownfield_init_scaffold_v0_1.md
---

# Brownfield Init Scaffold v0.1

## Summary

- Added explicit brownfield entry mode as `punk init <project-id> --mode brownfield`.
- Kept default `punk init <project-id>` greenfield behavior unchanged.
- Brownfield mode creates only compact `.punk/memory/` advisory reconstruction placeholders.
- Brownfield reconstruction remains not started; no repo scan, summaries, contracts/specs, claims, runtime storage, gate/proof runtime, or Writer behavior was added.

## Files Changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `docs/product/FLOW.md`
- `docs/product/DOGFOODING.md`
- `scripts/check_docs_governance.py`
- `evals/cases/docs/active-cli-surface/README.md`
- `work/STATUS.md`
- `work/goals/goal_add_brownfield_init_scaffold_v0_1.md`
- `work/goals/goal_verify_brownfield_init_scaffold_v0_1.md`
- `work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md`

## CLI Shape

```text
punk init <project-id> --mode brownfield
```

Default remains:

```text
punk init <project-id>
```

## Brownfield Scaffold

Brownfield mode creates:

```text
.punk/
  README.md
  project.toml
  memory/
    STATUS.md
    goals/
      goal_brownfield_reconstruction_baseline.md
    reports/
    reconstruction/
      README.md
      source-corpus-manifest.md
      claim-ledger.md
      unknowns.md
      contradictions.md
      contract-readiness.md
```

`.punk/project.toml` records:

```toml
entry_mode = "brownfield"

[brownfield]
reconstruction_status = "not_started"
authority = "advisory_candidates_only"
```

## Atomicity and Privacy

- Brownfield mode uses the same full preflight before writes as greenfield mode.
- If any brownfield scaffold conflict exists, init returns blocked and writes no planned missing artifacts.
- Human output renders `target_root: .` and does not expose absolute host paths.

## Pre-Commit Review Pass

- Overclaim grep: PASS. Targeted danger-pattern search found no phrasing that says Punk understands existing projects, reconstructs existing projects, creates project memory from existing code, generates contracts/specs, or creates brownfield claims.
- Docs-governance script diff: PASS. The script change only adds `punk init <project-id> --mode brownfield` to the implemented CLI command allowlist; it does not suppress warnings, broaden exceptions, or weaken validation.
- No-scan brownfield manual test: PASS. A non-empty temp project containing `src/app.rs`, `docs/README.md`, `secret-ish local content`, and `old docs` was initialized in brownfield mode; `rg` found none of those strings or paths under `.punk`.

## Tests Added

- `greenfield_default_behavior_unchanged`
- `brownfield_init_creates_compact_memory_scaffold`
- `brownfield_init_records_entry_mode_brownfield`
- `brownfield_init_records_reconstruction_not_started`
- `brownfield_init_creates_reconstruction_placeholders`
- `brownfield_init_does_not_scan_repo`
- `brownfield_init_does_not_create_runtime_dirs`
- `brownfield_init_does_not_create_contracts_or_claims`
- `brownfield_init_conflict_is_atomic_noop`
- `init_command_creates_brownfield_reconstruction_scaffold_when_requested`
- `init_command_rejects_unknown_mode`
- `eval_project_init_brownfield_scaffold_shape`

## Manual Verification

- Brownfield temp init: PASS. `~/.local/bin/punk-dev init demo-project --mode brownfield` created `.punk/memory/reconstruction/*`, `.punk/project.toml`, and `.punk/memory/STATUS.md`; `target_root: .` was rendered; `entry_mode = "brownfield"` and `reconstruction_status = "not_started"` were recorded; `git status --short --untracked-files=all` showed `.punk/memory/**` and `.punk/project.toml` as visible untracked files.
- Greenfield default temp init: PASS. `~/.local/bin/punk-dev init demo-project` still created the greenfield compact `.punk/memory/` scaffold with `entry_mode = "greenfield"`.
- Brownfield conflict temp init: PASS. A pre-existing modified `.punk/memory/STATUS.md` caused `result: blocked`, preserved the custom file, and created no other planned scaffold files.

## Boundaries Preserved

- No brownfield reconstruction was added.
- No repo scan was added.
- No AI summaries were added.
- No contract generation or spec generation was added.
- No gate/proof runtime was added.
- No Writer behavior was added.
- No Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added.
- No grayfield reconciliation was added.

## Checks Run

- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS; output includes `eval_project_init_brownfield_scaffold_shape`.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files README.md crates/punk-cli/src/main.rs crates/punk-eval/src/lib.rs crates/punk-project/src/lib.rs docs/product/CRATE-STATUS.md docs/product/DOGFOODING.md docs/product/FLOW.md docs/product/NORTH-STAR.md docs/product/PROJECT-MEMORY.md docs/product/ROADMAP.md docs/product/START-HERE.md evals/cases/docs/active-cli-surface/README.md scripts/check_docs_governance.py work/STATUS.md work/goals/goal_add_brownfield_init_scaffold_v0_1.md work/goals/goal_verify_brownfield_init_scaffold_v0_1.md work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md --report work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next Selected Goal

`work/goals/goal_verify_brownfield_init_scaffold_v0_1.md`

Reason: the next safe step is verification/status evidence for the new B0 scaffold, not inventory or reconstruction implementation.

## Doc impact
```yaml
  classification: code-doc
  reason: "Adds an explicit brownfield init scaffold CLI mode and updates current product docs while keeping reconstruction/runtime/contract behavior inactive."
```
