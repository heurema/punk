---
id: report_2026_04_26_eighteenth_work_ledger_review
goal_id: goal_run_eighteenth_work_ledger_review
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Run the eighteenth advisory Work Ledger Review after artifact hash policy helpers v0.1 landed in `punk-core`.

This review is advisory.
It does not decide acceptance.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

Explicit review conclusions:

- artifact hash policy helpers v0.1: **implemented and committed locally**;
- current implemented CLI truth: **`punk flow inspect`, `punk eval run smoke`, `punk eval run smoke --format json`**;
- setup neutrality: **preserved; no `punk init`, `.punk/`, local setup ritual, or executor requirement activated**;
- next safest branch: **add local smoke eval coverage for artifact hash policy helper behavior before `punk-proof` integration, proofpack writer, runtime storage, schema work, CLI behavior, or active hash computation**.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It changes only work-ledger artifacts and selects the next bounded goal.
Decision:
Proceed.

## Evidence Reviewed

Review range:

- `0a14f4e` - Run seventeenth Work Ledger Review
- `6a28e3f` - Add artifact hash policy helpers

Repo-tracked evidence:

- `work/STATUS.md`
- `work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md`
- `work/goals/goal_run_eighteenth_work_ledger_review.md`
- `work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md`
- `work/reports/2026-04-25-seventeenth-work-ledger-review.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-proof/src/lib.rs`
- `docs/product/DOGFOODING.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CRATE-STATUS.md`

## Findings

### F-001

```yaml
id: F-001
domain: core-helpers
finding: "`punk-core` now exposes dependency-free, side-effect-free artifact hash policy helpers with unit coverage for canonical digest format, invalid digest forms, repo-relative refs, invalid refs, and non-runtime capability flags."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-core/src/lib.rs
  - work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The helper boundary gives proof/eval code a shared place for policy-shaped validation instead of ad hoc duplicate checks."
revisit_trigger: "If later proof/eval code duplicates digest/ref validation instead of using shared helpers."
```

### F-002

```yaml
id: F-002
domain: eval
finding: "The smoke eval does not yet cover artifact hash policy helper behavior, so `punk eval run smoke` cannot catch regressions in AHP-style digest/ref validation."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-eval/src/lib.rs
  - crates/punk-eval/Cargo.toml
  - evals/specs/artifact-hash-policy.v0.1.md
  - crates/punk-core/src/lib.rs
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md
driver: vitaly
rationale_prevents_recurrence: "Adding smoke cases before proof integration keeps the active local eval surface aligned with the helper boundary."
revisit_trigger: "After smoke eval covers AHP helper behavior, or if helper behavior changes without smoke coverage."
```

### F-003

```yaml
id: F-003
domain: proof-integration
finding: "`punk-proof` still treats artifact hashes as non-empty strings and smoke/proof fixtures still use placeholder digest strings; direct proof integration should follow after smoke coverage or a separate bounded goal."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - crates/punk-proof/src/lib.rs
  - work/reports/2026-04-25-seventeenth-work-ledger-review.md
  - work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Deferring proof integration prevents mixing helper smoke coverage with proof API changes."
revisit_trigger: "After artifact hash policy helper smoke coverage exists."
```

### F-004

```yaml
id: F-004
domain: runtime-scope
finding: "Proofpack writer, `.punk` storage, active hash computation, schema files, gate/eval/proof orchestration, acceptance claims, and `punk init` remain larger than the next safe eval-coverage slice."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CRATE-STATUS.md
  - docs/product/DOGFOODING.md
  - work/STATUS.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Deferring runtime/writer/storage/orchestration keeps active-core honest while helper behavior gets eval coverage."
revisit_trigger: "After helper smoke coverage exists and a separate proof/runtime branch is selected."
```

### F-005

```yaml
id: F-005
domain: process
finding: "The Work Ledger still has one live status file and one selected next goal after artifact hash policy helper implementation."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - work/STATUS.md
  - docs/product/DOGFOODING.md
  - docs/product/PROJECT-MEMORY.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "One live ledger preserves setup-neutral project state while runtime Project Memory remains deferred."
revisit_trigger: "If multiple next goals are selected, a hidden planning source appears, or process-shell reuse becomes a second tracker."
```

## Candidate Improvements

### CI-001

```yaml
id: CI-001
based_on_findings:
  - F-002
summary: "Create `work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md` as the next bounded eval goal."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md
why_now: "Shared helpers exist and have unit tests; adding smoke coverage makes the active local eval surface guard the helper boundary before proof integration."
why_not_now: "Do not integrate `punk-proof`, compute hashes, add schemas, write proofpacks, write `.punk` storage, or expose CLI behavior in this step."
driver: vitaly
```

### CI-002

```yaml
id: CI-002
based_on_findings:
  - F-003
summary: "Defer `punk-proof` helper integration until after AHP helper smoke coverage or another explicit bounded proof goal."
priority: medium
destination:
  type: no-action
  ref: ""
why_now: "Proof integration changes the proof API/fixtures and should not be mixed with smoke coverage selection."
why_not_now: "Keep the next diff small and eval-focused."
driver: vitaly
```

## Parked Ideas

- `punk-proof` integration with artifact hash policy helpers
- active artifact hash computation
- hash normalization implementation
- proofpack writer implementation
- runtime Project Memory storage
- `.punk/contracts`, `.punk/evals`, `.punk/runs`, `.punk/decisions`, and `.punk/proofs` activation
- gate/eval/proof orchestration
- schema files
- acceptance claim schema or implementation
- semantic assessor implementation
- GoalRail runtime pilot
- possible future `punk init`
- service-backed project memory

## No Action

- Do not change product CLI in this review.
- Do not implement `punk init`.
- Do not write `.punk/` state.
- Do not implement hash computation or normalization.
- Do not add a hashing dependency.
- Do not implement proofpack writer behavior yet.
- Do not implement schema files.
- Do not integrate `punk-proof` in this review.
- Do not claim acceptance from receipt, gate, proofpack kernel, proofpack integrity helper, hash policy, helper validation, or smoke eval.
- Do not fold GoalRail runtime work into Punk's active-core scope.
- Do not add provider/model/agent adapters.

## Next Recommended Action

Recommend: `work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md`
Defer: `punk-proof` integration, active hash computation, proofpack writer, runtime storage, gate/eval/proof orchestration, schema files, acceptance claim work, semantic assessor implementation, GoalRail runtime work, `punk init`
Park: service-backed storage as authority

Rationale:

- artifact hash policy helpers now exist and have unit tests;
- smoke eval is the current active local assessment surface;
- adding smoke coverage is narrower than proof integration or writer/runtime work;
- proof/eval integration can reuse helpers later instead of duplicating validation semantics.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds a bounded advisory review and work-ledger artifacts without changing canonical product docs, specs, research notes, runtime code, schemas, or CLI behavior."
  touched_surfaces:
    - work/reports/2026-04-26-eighteenth-work-ledger-review.md
    - work/goals/goal_run_eighteenth_work_ledger_review.md
    - work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md
    - work/STATUS.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_run_eighteenth_work_ledger_review.md work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md work/reports/2026-04-26-eighteenth-work-ledger-review.md --report work/reports/2026-04-26-eighteenth-work-ledger-review.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS on final run
  - Note: repeated validation runs exposed a `punk-events` test temp path collision in `file_backed_writer_leaves_deterministic_test_evidence`; handled separately by `work/goals/goal_fix_punk_events_test_temp_path_collision.md`.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
