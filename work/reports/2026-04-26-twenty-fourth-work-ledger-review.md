---
id: report_2026_04_26_twenty_fourth_work_ledger_review
goal_id: goal_run_twenty_fourth_work_ledger_review
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Run the twenty-fourth advisory Work Ledger Review after artifact hash computation helper v0.1 was implemented.

This review is advisory.
It does not decide acceptance.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

Explicit review conclusions:

- exact-byte hash computation: **implemented in `punk-core` as `compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest`**;
- dependency boundary: **`punk-core` now owns a narrow `sha2` dependency for SHA-256 computation; no dependency types are exposed through public Punk APIs**;
- smoke coverage: **`punk-eval` now includes known-vector and exact-byte preservation smoke cases**;
- current implemented CLI truth: **`punk flow inspect`, `punk eval run smoke`, `punk eval run smoke --format json`**;
- setup neutrality: **preserved; no IDE, CLI ritual, local runtime setup, provider, model, skill, adapter, automation, `.punk/` state, or `punk init` activated**;
- next safest branch: **reconcile `docs/product/CRATE-STATUS.md` currentness before selecting proofpack writer preparation, file IO hashing, runtime storage, schema files, CLI behavior, gate/eval/proof orchestration, adapters, automation, or `punk init`**.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It changes only work-ledger artifacts and selects the next bounded goal.
Decision:
Proceed.

## Evidence Reviewed

Review range:

- `30589d8` - Add artifact hash computation helper

Repo-tracked evidence:

- `work/STATUS.md`
- `work/goals/goal_add_artifact_hash_computation_helper_v0_1.md`
- `work/goals/goal_run_twenty_fourth_work_ledger_review.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md`
- `work/reports/2026-04-26-twenty-third-work-ledger-review.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
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
domain: hash-computation
finding: "Artifact hash computation helper v0.1 is now active in `punk-core` as a side-effect-free exact-byte SHA-256 helper over caller-provided bytes."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-core/src/lib.rs
  - crates/punk-core/Cargo.toml
  - work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The implementation keeps hashing narrow: no file IO, no byte normalization, no runtime writes, no proofpack writer, no gate decision writer, and no CLI behavior."
revisit_trigger: "If a future hash helper reads files, maps refs, normalizes bytes, writes runtime state, or participates in gate/proof acceptance."
```

### F-002

```yaml
id: F-002
domain: docs-currentness
finding: "`docs/product/CRATE-STATUS.md` still says `punk-core` is dependency-free and does not compute hashes, which is stale after the exact-byte hash computation helper landed with a narrow `sha2` dependency."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - docs/product/CRATE-STATUS.md
  - crates/punk-core/Cargo.toml
  - work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md
driver: vitaly
rationale_prevents_recurrence: "A docs-only currentness goal can correct canonical crate-status wording without expanding runtime, writer, schema, or CLI scope."
revisit_trigger: "After CRATE-STATUS records exact-byte hash computation and the remaining deferred hash/proof/runtime boundaries accurately."
```

### F-003

```yaml
id: F-003
domain: eval-coverage
finding: "Smoke eval now covers exact-byte hash computation known vectors and exact-byte preservation, while file IO hashing and runtime report storage remain deferred."
severity: medium
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-eval/src/lib.rs
  - work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Local smoke coverage protects the helper shape without claiming runtime eval reports, baselines, waivers, or gate acceptance."
revisit_trigger: "If exact-byte hash computation changes, if file IO hashing is introduced, or if runtime eval report storage becomes selected work."
```

### F-004

```yaml
id: F-004
domain: runtime-scope
finding: "Proofpack writer behavior, file IO hashing, byte normalization, `.punk` storage, schemas, CLI behavior, gate/eval/proof orchestration, adapters, automation, and `punk init` remain intentionally deferred."
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
rationale_prevents_recurrence: "Keeping the next step docs-currentness only prevents the helper from becoming a hidden proof/runtime system."
revisit_trigger: "After CRATE-STATUS reconciliation selects a new bounded implementation or boundary goal."
```

### F-005

```yaml
id: F-005
domain: process
finding: "The Work Ledger still has one live status file and one selected next goal after exact-byte hash computation."
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
summary: "Create `work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md` as the next bounded docs-currentness goal."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md
why_now: "CRATE-STATUS is canonical for active-core crate boundaries and currently contradicts implemented `punk-core` hash computation behavior."
why_not_now: "Do not use this reconciliation to add proofpack writer behavior, file IO hashing, `.punk` runtime storage, schema files, CLI behavior, or gate/eval/proof orchestration."
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

`work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md`

Rationale:
The exact-byte hash computation helper is now implemented. The canonical crate-status doc must stop saying `punk-core` is dependency-free and does not compute hashes before Punk selects writer, file IO, runtime, schema, CLI, or orchestration work.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

The review did not:

- change Rust code;
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
- `work/goals/goal_run_twenty_fourth_work_ledger_review.md`
- `work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md`
- `work/reports/2026-04-26-twenty-fourth-work-ledger-review.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Advisory Work Ledger Review updated work status, completed the review goal, created the next docs-currentness goal, and recorded a handoff report."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_run_twenty_fourth_work_ledger_review.md
    - work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md
    - work/reports/2026-04-26-twenty-fourth-work-ledger-review.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_run_twenty_fourth_work_ledger_review.md work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md work/reports/2026-04-26-twenty-fourth-work-ledger-review.md --report work/reports/2026-04-26-twenty-fourth-work-ledger-review.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
