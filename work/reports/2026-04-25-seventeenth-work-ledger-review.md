---
id: report_2026_04_25_seventeenth_work_ledger_review
goal_id: goal_run_seventeenth_work_ledger_review
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Summary

Run the seventeenth advisory Work Ledger Review after artifact hash policy v0.1 landed.

This review is advisory.
It does not decide acceptance.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

Explicit review conclusions:

- artifact hash policy v0.1: **implemented and committed locally**;
- current implemented CLI truth: **`punk flow inspect`, `punk eval run smoke`, `punk eval run smoke --format json`**;
- setup neutrality: **preserved; no `punk init`, `.punk/`, local setup ritual, or executor requirement activated**;
- next safest branch: **add side-effect-free artifact hash policy validation helpers in `punk-core` before proofpack integration, smoke eval coverage, writer/runtime/storage/schema work, or CLI behavior**.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It changes only work-ledger artifacts and selects the next bounded goal.
Decision:
Proceed.

## Evidence Reviewed

Review range:

- `547a036` - Run sixteenth Work Ledger Review
- `5ce24e1` - Define artifact hash policy

Repo-tracked evidence:

- `work/STATUS.md`
- `work/goals/goal_define_artifact_hash_policy_v0_1.md`
- `work/goals/goal_run_seventeenth_work_ledger_review.md`
- `work/reports/2026-04-25-artifact-hash-policy-v0-1.md`
- `work/reports/2026-04-25-sixteenth-work-ledger-review.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `docs/product/DOGFOODING.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CRATE-STATUS.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-proof/src/lib.rs`

## Findings

### F-001

```yaml
id: F-001
domain: hash-policy
finding: "Artifact hash policy v0.1 defines canonical `sha256:<64 lowercase hex chars>` digests, repo-relative artifact ref boundaries, exact-byte hashing boundaries, and visible missing/invalid/unsupported/ref-mismatch/unverified digest states."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - evals/specs/artifact-hash-policy.v0.1.md
  - work/reports/2026-04-25-artifact-hash-policy-v0-1.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The policy prevents later writer/runtime work from inventing implicit hash normalization or accepting executor digest claims as proof."
revisit_trigger: "If a later implementation computes hashes, validates digest strings, or writes proofpacks without referencing the policy."
```

### F-002

```yaml
id: F-002
domain: core-helpers
finding: "`punk-core` is still a minimal helper skeleton, so there is no side-effect-free code boundary that validates policy-shaped digests or repo-relative artifact refs before proofpack integration."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-core/src/lib.rs
  - docs/product/CRATE-STATUS.md
  - evals/specs/artifact-hash-policy.v0.1.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md
driver: vitaly
rationale_prevents_recurrence: "A small `punk-core` helper slice makes the policy executable as validation without adding hashing dependencies, proofpack writer behavior, schemas, runtime storage, or CLI commands."
revisit_trigger: "After helper validation exists, or if proofpack/eval code starts duplicating hash/ref validation logic."
```

### F-003

```yaml
id: F-003
domain: proof-integration
finding: "`punk-proof` currently treats artifact hashes as non-empty strings and uses placeholder hash values in tests; direct proof integration should wait until shared policy helpers exist."
severity: medium
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - crates/punk-proof/src/lib.rs
  - evals/specs/artifact-hash-policy.v0.1.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Deferring proof integration avoids spreading ad hoc digest validation into the proof crate before the shared helper boundary exists."
revisit_trigger: "After `punk-core` exposes hash/ref policy helpers."
```

### F-004

```yaml
id: F-004
domain: runtime-scope
finding: "Proofpack writer, `.punk` storage, active hash computation, schema files, gate/eval/proof orchestration, acceptance claims, and `punk init` remain larger than the next safe helper slice."
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
rationale_prevents_recurrence: "Deferring runtime/writer/storage/orchestration keeps active-core honest while hash policy is converted into side-effect-free helpers."
revisit_trigger: "After helpers and possibly smoke eval coverage exist."
```

### F-005

```yaml
id: F-005
domain: process
finding: "The Work Ledger still has one live status file and one selected next goal after artifact hash policy v0.1."
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
summary: "Create `work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md` as the next bounded implementation goal."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md
why_now: "The policy is stable enough to implement digest/ref validation helpers without computing hashes or touching proofpack/runtime/CLI surfaces."
why_not_now: "Do not integrate proofpack, compute hashes, add dependencies, write proofpacks, write `.punk` storage, add schemas, or expose CLI behavior in this step."
driver: vitaly
```

### CI-002

```yaml
id: CI-002
based_on_findings:
  - F-003
summary: "Defer `punk-proof` integration and smoke eval coverage until after the shared `punk-core` helper boundary exists."
priority: medium
destination:
  type: no-action
  ref: ""
why_now: "Shared helpers avoid duplicating digest/ref validation semantics in proof/eval crates."
why_not_now: "Proof integration should be a separate bounded goal after helpers exist."
driver: vitaly
```

## Parked Ideas

- active artifact hash computation
- hash normalization implementation
- proofpack integration with hash policy helpers
- proofpack integrity smoke eval coverage for AHP cases
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
- Do not claim acceptance from receipt, gate, proofpack kernel, proofpack integrity helper, hash policy, or smoke eval.
- Do not fold GoalRail runtime work into Punk's active-core scope.
- Do not add provider/model/agent adapters.

## Next Recommended Action

Recommend: `work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md`
Defer: active hash computation, proofpack integration, proofpack writer, runtime storage, gate/eval/proof orchestration, schema files, acceptance claim work, semantic assessor implementation, GoalRail runtime work, `punk init`
Park: service-backed storage as authority

Rationale:

- artifact hash policy now defines the boundary;
- `punk-core` is the intended crate for deterministic helpers and future hashing helpers;
- a side-effect-free validation helper slice is narrower than proofpack integration, smoke eval coverage, writer, storage, CLI, schema, or real hash computation;
- proof/eval integration can reuse helpers later instead of duplicating validation semantics.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds a bounded advisory review and work-ledger artifacts without changing canonical product docs, specs, research notes, runtime code, schemas, or CLI behavior."
  touched_surfaces:
    - work/reports/2026-04-25-seventeenth-work-ledger-review.md
    - work/goals/goal_run_seventeenth_work_ledger_review.md
    - work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md
    - work/STATUS.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_run_seventeenth_work_ledger_review.md work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md work/reports/2026-04-25-seventeenth-work-ledger-review.md --report work/reports/2026-04-25-seventeenth-work-ledger-review.md` - PASS.
- `cargo test --workspace` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.
