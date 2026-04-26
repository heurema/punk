---
id: report_2026_04_26_twenty_third_work_ledger_review
goal_id: goal_run_twenty_third_work_ledger_review
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Run the twenty-third advisory Work Ledger Review after artifact hash computation helper boundary v0.1 was defined.

This review is advisory.
It does not decide acceptance.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

Explicit review conclusions:

- artifact hash computation helper boundary: **defined as exact caller-provided bytes to canonical `sha256:<64 lowercase hex>` digest metadata**;
- dependency stance: **Rust stdlib lacks SHA-256; Punk should not hand-roll crypto; future implementation may add one narrow maintained SHA-256 dependency in `punk-core` only**;
- current implemented CLI truth: **`punk flow inspect`, `punk eval run smoke`, `punk eval run smoke --format json`**;
- setup neutrality: **preserved; no IDE, CLI ritual, local runtime setup, provider, model, skill, adapter, automation, `.punk/` state, or `punk init` activated**;
- next safest branch: **implement the side-effect-free exact-byte `punk-core` hash computation helper v0.1 plus smoke eval coverage before proofpack writer, file IO hashing, runtime storage, schema files, CLI behavior, or gate/eval/proof orchestration**.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It changes only work-ledger artifacts and selects the next bounded goal.
Decision:
Proceed.

## Evidence Reviewed

Review range:

- `d0a4fe7` - Define artifact hash computation helper boundary

Repo-tracked evidence:

- `work/STATUS.md`
- `work/goals/goal_define_artifact_hash_computation_helper_boundary_v0_1.md`
- `work/goals/goal_run_twenty_third_work_ledger_review.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md`
- `work/reports/2026-04-26-twenty-second-work-ledger-review.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `docs/product/DOGFOODING.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CRATE-STATUS.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-core/Cargo.toml`
- `crates/punk-eval/src/lib.rs`

## Findings

### F-001

```yaml
id: F-001
domain: hash-boundary
finding: "Artifact hash computation helper boundary v0.1 now defines the safe implementation seam: exact caller-provided bytes in, canonical SHA-256 digest metadata out, with file IO, path/ref policy, byte normalization, runtime storage, CLI, and proofpack writer behavior out of scope."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - evals/specs/artifact-hash-computation-helper.v0.1.md
  - work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The boundary prevents hash implementation from silently absorbing file IO, normalization, storage, proofpack writer, or CLI scope."
revisit_trigger: "If a future hash helper implementation changes exact-byte semantics or adds file/runtime behavior."
```

### F-002

```yaml
id: F-002
domain: dependency-boundary
finding: "The dependency stance is explicit enough for a bounded implementation: stdlib lacks SHA-256, hand-rolled crypto is rejected, and a future implementation may add one narrow maintained SHA-256 dependency in `punk-core` only."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - evals/specs/artifact-hash-computation-helper.v0.1.md
  - crates/punk-core/Cargo.toml
  - docs/product/CRATE-STATUS.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_computation_helper_v0_1.md
driver: vitaly
rationale_prevents_recurrence: "A small implementation goal can add the helper and dependency without involving proofpack writer, runtime storage, or CLI behavior."
revisit_trigger: "After `punk-core` can compute exact-byte digests, or if implementation discovers dependency scope cannot stay narrow."
```

### F-003

```yaml
id: F-003
domain: eval-coverage
finding: "Smoke eval currently covers digest/ref validation and proofpack manifest rendering, but not exact-byte SHA-256 computation because computation is not implemented yet."
severity: medium
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-eval/src/lib.rs
  - evals/specs/artifact-hash-computation-helper.v0.1.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_computation_helper_v0_1.md
driver: vitaly
rationale_prevents_recurrence: "The implementation goal should include smoke eval coverage for empty bytes, known bytes, canonical output, exact-byte preservation, and side-effect-free behavior."
revisit_trigger: "After helper implementation and smoke eval coverage are in place."
```

### F-004

```yaml
id: F-004
domain: runtime-scope
finding: "File IO hashing, proofpack writer, `.punk` storage, schemas, CLI behavior, gate/eval/proof orchestration, acceptance claims, adapters, automation, and `punk init` remain larger than the next safe helper slice."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - docs/product/DOGFOODING.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CRATE-STATUS.md
  - evals/specs/artifact-hash-computation-helper.v0.1.md
  - work/STATUS.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Keeping the next implementation helper-only preserves active-core scope and setup neutrality."
revisit_trigger: "After exact-byte hash computation helper and smoke eval coverage are complete."
```

### F-005

```yaml
id: F-005
domain: process
finding: "The Work Ledger still has one live status file and one selected next goal after the hash computation helper boundary."
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
rationale_prevents_recurrence: "One live ledger preserves inspectable Level 0 state while runtime Project Memory remains deferred."
revisit_trigger: "If multiple next goals are selected, hidden planning state appears, or process-shell reuse becomes a second tracker."
```

## Candidate Improvements

### CI-001

```yaml
id: CI-001
based_on_findings:
  - F-002
  - F-003
summary: "Create `work/goals/goal_add_artifact_hash_computation_helper_v0_1.md` as the next bounded implementation goal."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_computation_helper_v0_1.md
why_now: "The hash policy, validation helpers, proofpack manifest renderer, and exact-byte computation boundary are now in place."
why_not_now: "Do not add file IO hashing, proofpack writer, `.punk` runtime storage, schemas, CLI behavior, or gate/eval/proof orchestration in the helper implementation."
driver: vitaly
```

## Parked Ideas

- file IO artifact hashing helper
- proofpack writer implementation
- byte or hash normalization implementation
- runtime Project Memory storage
- `.punk/contracts`, `.punk/evals`, `.punk/runs`, `.punk/decisions`, and `.punk/proofs` activation
- gate/eval/proof orchestration
- schema files
- acceptance claim schema or implementation
- semantic assessor implementation
- GoalRail runtime pilot
- possible future `punk init`
- service-backed project memory

## Selected Next

`work/goals/goal_add_artifact_hash_computation_helper_v0_1.md`

Rationale:
The exact-byte hash computation helper boundary is now explicit. The next smallest active-core implementation is a side-effect-free `punk-core` helper that computes canonical SHA-256 digests from caller-provided bytes, with smoke eval coverage and no file IO/runtime/writer/CLI behavior.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

The review did not:

- implement hash computation;
- add dependencies;
- add file IO;
- normalize bytes or hashes;
- write gate decisions;
- write proofpacks;
- write runtime storage;
- add schema files;
- add CLI commands;
- add adapters, automation, provider/model runners, or agent integrations;
- implement `punk init`.

## Changed Files

- `work/STATUS.md`
- `work/goals/goal_run_twenty_third_work_ledger_review.md`
- `work/goals/goal_add_artifact_hash_computation_helper_v0_1.md`
- `work/reports/2026-04-26-twenty-third-work-ledger-review.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Advisory Work Ledger Review updated work status, completed the review goal, created the next implementation goal, and recorded a handoff report."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_run_twenty_third_work_ledger_review.md
    - work/goals/goal_add_artifact_hash_computation_helper_v0_1.md
    - work/reports/2026-04-26-twenty-third-work-ledger-review.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_run_twenty_third_work_ledger_review.md work/goals/goal_add_artifact_hash_computation_helper_v0_1.md work/reports/2026-04-26-twenty-third-work-ledger-review.md --report work/reports/2026-04-26-twenty-third-work-ledger-review.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
