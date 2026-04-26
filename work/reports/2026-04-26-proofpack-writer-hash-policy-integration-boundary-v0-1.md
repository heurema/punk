---
id: report_2026_04_26_proofpack_writer_hash_policy_integration_boundary_v0_1
goal_id: goal_define_proofpack_writer_hash_policy_integration_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Defined proofpack writer hash-policy integration boundary v0.1 as docs/spec only.

The new boundary explains how a future proofpack writer may preserve hash-related evidence without activating writer/runtime authority:

- declared artifact digests remain explicit inputs and must satisfy artifact hash policy;
- structural link/hash integrity remains separate from referenced artifact byte verification;
- referenced artifact verification outcomes must be preserved without reinterpretation;
- manifest self-digest identifies manifest bytes only and does not verify referenced artifacts;
- missing, mismatched, unreadable, unverified, invalid, and unsupported states stay visible;
- setup neutrality is preserved and no IDE, CLI ritual, local runtime state, provider, model, skill, adapter, automation, or `punk init` is required.

No Rust code, `.punk/` runtime state, schema files, CLI behavior, proofpack writer behavior, proofpack referenced-ref verification integration implementation, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` were added.

## Research Gate

Classification: R1
Required: yes
Rationale:
This was a bounded docs/spec boundary derived from repo-tracked proofpack, hash, file IO, and referenced-artifact verification artifacts. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `docs/product/ROADMAP.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-writer-preparation-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `evals/specs/referenced-artifact-verification-boundary.v0.1.md`
- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `crates/punk-proof/src/lib.rs`
- `crates/punk-core/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-proofpack-writer-preparation-boundary-v0-1.md`
- `work/reports/2026-04-26-thirty-fifth-work-ledger-review.md`

## Changed Files

- `evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_proofpack_writer_hash_policy_integration_boundary_v0_1.md`
- `work/goals/goal_run_thirty_sixth_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md`

## What changed

- Added a proofpack writer hash-policy integration docs/spec boundary.
- Defined the four separate hash-related surfaces: declared artifact digests, structural link/hash integrity, referenced artifact verification outcomes, and manifest self-digest.
- Defined future declared digest statuses and verification outcome propagation behavior.
- Recorded that `verified` remains evidence only and that `digest_mismatch`, `missing`, unreadable, invalid, unsupported, and `unverified` states must remain visible.
- Clarified that manifest self-digest does not satisfy referenced artifact digests or byte verification.
- Preserved setup-neutral boundaries and future-only authority limits.
- Added the thirty-sixth advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI/`.punk` authority changed.

This did not:

- change Rust code;
- add dependencies;
- write `.punk/` state;
- add schema files;
- add CLI behavior;
- implement proofpack writer behavior;
- implement proofpack referenced-ref verification integration;
- normalize bytes or hashes;
- broaden file IO hashing;
- write proofpacks;
- write gate decisions;
- add acceptance claims;
- add provider/model/agent adapters;
- add automation;
- implement `punk init`;
- claim acceptance.

The current implemented CLI surface remains:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Added a docs/spec boundary for proofpack writer hash-policy integration without promoting proofpack writer, runtime, schema, CLI, gate, or acceptance scope."
  touched_surfaces:
    - evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_proofpack_writer_hash_policy_integration_boundary_v0_1.md
    - work/goals/goal_run_thirty_sixth_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md
  required_updates:
    - evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md
    - work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md work/STATUS.md work/goals/goal_define_proofpack_writer_hash_policy_integration_boundary_v0_1.md work/goals/goal_run_thirty_sixth_work_ledger_review.md work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md --report work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md` - PASS with 0 failures and 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the thirty-sixth advisory Work Ledger Review.
- Decide whether the next branch should be proofpack writer implementation boundary, proofpack writer storage/schema boundary, proofpack referenced-ref verification integration implementation, additional smoke eval/docs guardrails, or another active-core setup step.
- Keep proofpack writer behavior, `.punk/` storage, runtime gate/proof behavior, schemas, CLI expansion, adapters, automation, provider/model runners, and `punk init` deferred until separately selected.
