---
id: report_2026_05_21_module_control_manifest_and_tuning_proposal_contract_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1.md
---

# Module Control Manifest and Tuning Proposal Contract v0.1

## Summary

Defined the module control manifest and tuning proposal contract.

This turns the adaptive module control-plane direction into a concrete docs/eval
surface while keeping all runtime behavior parked.

## Files changed

- `docs/product/MODULE-CONTROL-PLANE.md`
- `docs/product/MODULE-AUTHORING.md`
- `docs/product/MODULE-CONFORMANCE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `evals/specs/module-control-plane.v0.1.md`
- `work/goals/goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1.md`
- `work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added the canonical Module Control Plane product doc.
- Defined module control manifest shape.
- Defined behavior artifact refs and immutable core boundaries.
- Defined tuning proposal shape.
- Added PubPunk as an advisory fixture with no active runtime behavior.
- Added deterministic eval/spec cases.
- Linked the new contract from module authoring, module conformance, and the
  documentation map.

## Boundary confirmation

- Reflection remains propose-only.
- `auto_apply` is false in the contract fixture.
- Behavior artifacts cannot grant capabilities.
- Behavior artifacts cannot rewrite immutable core boundaries.
- Tuning proposals cannot silently change side effects.
- PubPunk remains a fixture only.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  canonical_docs:
    - docs/product/MODULE-CONTROL-PLANE.md
    - docs/product/MODULE-AUTHORING.md
    - docs/product/MODULE-CONFORMANCE.md
    - docs/product/DOCUMENTATION-MAP.md
  evals:
    - evals/specs/module-control-plane.v0.1.md
  work_artifacts:
    - work/goals/goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1.md
    - work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md
    - work/STATUS.md
  reason: "Adds the canonical docs/eval contract for module control manifests and tuning proposals without activating runtime, adapters, reflection automation, behavior-artifact writers, publishing, metrics, credential access, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files docs/product/MODULE-CONTROL-PLANE.md docs/product/MODULE-AUTHORING.md docs/product/MODULE-CONFORMANCE.md docs/product/DOCUMENTATION-MAP.md evals/specs/module-control-plane.v0.1.md work/goals/goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1.md work/STATUS.md --report work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md` - PASS.
- `git diff --check` - PASS.
- `cargo check --workspace` - not run; no Rust code changed.

## Knowledge impact

- Canonical product docs changed: `docs/product/MODULE-CONTROL-PLANE.md`,
  `docs/product/MODULE-AUTHORING.md`,
  `docs/product/MODULE-CONFORMANCE.md`, and
  `docs/product/DOCUMENTATION-MAP.md`.
- Project-memory claims affected: future module control work now has a
  canonical contract for behavior artifacts and tuning proposals.
- Docs / ADRs / evals possibly stale: none identified; future runtime
  promotion would need separate ADR/eval/goal work.
- Active / parked / future scope affected: active runtime scope unchanged;
  runtime manifest parsing, reflection scheduling, behavior-artifact writing,
  module loading, adapter invocation, and gate/proof behavior remain parked.
- Public narrative impact: none.
- Follow-up goals or drift findings: a future slice may instantiate a PubPunk
  module control manifest fixture or draft a non-applying tuning proposal
  example against real evidence.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, deterministic
checker implementation, Module Host runtime activation, PubPunk runtime
activation, `.punk` runtime state, background reflection automation,
behavior-artifact writer, adapter invocation, browser automation, credential
access, external publishing, metrics collection, gate writer, proofpack writer,
or acceptance claim was added.
