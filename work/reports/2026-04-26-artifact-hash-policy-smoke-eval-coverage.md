---
id: report_2026_04_26_artifact_hash_policy_smoke_eval_coverage
goal_id: goal_add_artifact_hash_policy_smoke_eval_coverage
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Added deterministic local smoke eval coverage for artifact hash policy helper behavior.

This extends the existing `punk eval run smoke` assessment surface without adding CLI commands, runtime storage, gate decisions, proofpacks, schemas, adapters, automation, active hash computation, byte normalization, or `punk init`.

## Research Gate

Classification: R0
Required: no
Rationale:
This change adds deterministic smoke eval coverage over already-defined and implemented side-effect-free `punk-core` artifact hash policy helpers.
Decision:
Proceed.

## Changed Files

- `Cargo.lock`
- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md`
- `work/goals/goal_run_nineteenth_work_ledger_review.md`
- `work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md`

## Implementation

- Added `punk-core` as a `punk-eval` dependency so smoke eval uses the shared helper boundary instead of duplicating validation rules.
- Added smoke cases for:
  - canonical SHA-256 digest acceptance;
  - invalid digest rejection;
  - valid repo-relative artifact ref acceptance;
  - invalid artifact ref rejection;
  - helper capability flags staying validation-only.
- Updated smoke summary, human output assertions, JSON output assertions, boundary notes, and deferred notes to include artifact hash policy helper coverage.

## Boundary

No runtime/code authority expansion:

- no `.punk/evals` write;
- no gate decision write;
- no proofpack writer;
- no acceptance claim;
- no runtime storage;
- no schema file;
- no active hash computation;
- no byte normalization;
- no CLI command addition;
- no provider/model/agent adapter;
- no automation;
- no `punk init`.

The smoke eval remains local assessment only.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Active smoke eval behavior changed and a work report records the bounded implementation; canonical product docs/specs did not change."
  touched_surfaces:
    - Cargo.lock
    - crates/punk-eval/Cargo.toml
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md
    - work/goals/goal_run_nineteenth_work_ledger_review.md
    - work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `cargo fmt --all` - PASS
- `cargo test -p punk-eval` - PASS
- `cargo run -p punk-cli -- eval run smoke` - PASS
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files Cargo.lock crates/punk-eval/Cargo.toml crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md work/goals/goal_run_nineteenth_work_ledger_review.md work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md --report work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md` - PASS, 0 failures, 0 warnings
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
