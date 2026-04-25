---
id: report_2026_04_25_proof_before_acceptance_semantics_v0_1
goal_id: goal_define_proof_before_acceptance_semantics_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Define proof-before-acceptance semantics v0.1 as a docs/spec-only artifact before any runtime gate/proof/proofpack implementation.

## Research Gate

Classification: R1
Required: yes
Rationale:
The semantics artifact reconciles existing canonical laws, architecture, roadmap, gate boundary, proofpack boundary, receipt fields, missing-validator policy, and semantic assessor boundary. No new Deep Research was required.

Research refs:

- `docs/product/RESEARCH-GATE.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/minimal-receipt-fields.v0.1.md`
- `evals/specs/missing-validator-policy.v0.1.md`
- `evals/specs/semantic-assessor-command-interface.v0.1.md`
- `work/reports/2026-04-25-sixth-work-ledger-review.md`

Decision:
Proceed with docs/spec/work-ledger artifacts only.

## Changed files

- `docs/product/START-HERE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/minimal-receipt-fields.v0.1.md`
- `evals/specs/missing-validator-policy.v0.1.md`
- `evals/specs/semantic-assessor-command-interface.v0.1.md`
- `work/goals/goal_define_proof_before_acceptance_semantics_v0_1.md`
- `work/goals/goal_run_seventh_work_ledger_review.md`
- `work/reports/2026-04-25-proof-before-acceptance-semantics-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `evals/specs/proof-before-acceptance-semantics.v0.1.md`.
- Clarified in canonical docs that gate writes the decision and proof/proofpack makes the decision inspectable before acceptance.
- Linked the semantics from gate decision, proofpack, receipt fields, missing-validator policy, and semantic assessor specs.
- Closed the selected proof-before-acceptance semantics goal with report/spec refs.
- Added the seventh advisory Work Ledger Review as the next ready goal.
- Updated `work/STATUS.md` so the single live ledger selects the seventh review.

## Semantics summary

Core reconciliation:

```text
Only gate writes the final decision.
Proof comes before acceptance.
```

Artifact separation:

- gate decision = final authority over evidence;
- proofpack = post-decision provenance that makes the decision inspectable;
- acceptance claim = requires accepting gate decision plus matching proof.

Safe shorthand:

```text
decide with gate
prove the decision
then claim acceptance
```

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk` runtime state written;
- no runtime gate behavior added;
- no proofpack writer added;
- no receipt schema/runtime added;
- no semantic assessor implementation added;
- no CLI command added;
- no `punk init` command added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Proof-before-acceptance semantics v0.1 clarifies canonical closure sequence and authority boundaries before implementation."
  touched_surfaces:
    - docs/product/START-HERE.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - evals/specs/proof-before-acceptance-semantics.v0.1.md
    - evals/specs/gate-decision-boundary.v0.1.md
    - evals/specs/proofpack-boundary.v0.1.md
    - evals/specs/minimal-receipt-fields.v0.1.md
    - evals/specs/missing-validator-policy.v0.1.md
    - evals/specs/semantic-assessor-command-interface.v0.1.md
    - work/goals/goal_define_proof_before_acceptance_semantics_v0_1.md
    - work/goals/goal_run_seventh_work_ledger_review.md
    - work/reports/2026-04-25-proof-before-acceptance-semantics-v0-1.md
    - work/STATUS.md
  required_updates:
    - docs/product/START-HERE.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - evals/specs/proof-before-acceptance-semantics.v0.1.md
    - evals/specs/gate-decision-boundary.v0.1.md
    - evals/specs/proofpack-boundary.v0.1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future implementation must keep gate decision, proofpack, and acceptance claim separate."
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_run_seventh_work_ledger_review.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_run_seventh_work_ledger_review.md`; goals checked: 55.
- `scripts/check.sh docs-governance --files docs/product/START-HERE.md docs/product/ARCHITECTURE.md docs/product/ROADMAP.md evals/specs/proof-before-acceptance-semantics.v0.1.md evals/specs/gate-decision-boundary.v0.1.md evals/specs/proofpack-boundary.v0.1.md evals/specs/minimal-receipt-fields.v0.1.md evals/specs/missing-validator-policy.v0.1.md evals/specs/semantic-assessor-command-interface.v0.1.md work/goals/goal_define_proof_before_acceptance_semantics_v0_1.md work/goals/goal_run_seventh_work_ledger_review.md work/reports/2026-04-25-proof-before-acceptance-semantics-v0-1.md work/STATUS.md --report work/reports/2026-04-25-proof-before-acceptance-semantics-v0-1.md` - PASS; changed files: 13; canonical docs checked: 3; reports checked: 1; failures: 0; warnings: 5 existing duplicate-definition candidates in canonical docs.
- `cargo test --workspace` - PASS; Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

## Open follow-ups

- Run the seventh advisory Work Ledger Review.
- Implement runtime gate only through a later bounded goal.
- Implement proofpack writer only through a later bounded goal.
- Implement receipt schema/runtime only through a later bounded goal.
- Implement real `.punk` runtime storage only through a later bounded goal.
- Revisit possible future `punk init` only when runtime setup is intentionally scoped.
