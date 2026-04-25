---
id: report_2026_04_25_gate_decision_kernel_minimal_v0_1
goal_id: goal_add_gate_decision_kernel_minimal_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add a minimal side-effect-free gate decision kernel v0.1 so Punk can model final decision authority without activating runtime gate storage, CLI, proofpacks, or acceptance claims.

## Research Gate

Classification: R0
Required: no
Rationale:
This implements already-defined gate decision and proof-before-acceptance boundaries in the existing Rust gate crate. No new architecture decision or external research was required.

Research refs:

- `evals/specs/gate-decision-boundary.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/minimal-receipt-fields.v0.1.md`
- `work/reports/2026-04-25-tenth-work-ledger-review.md`

Decision:
Proceed with a side-effect-free gate kernel implementation only.

## Changed files

- `crates/punk-gate/src/lib.rs`
- `work/goals/goal_add_gate_decision_kernel_minimal_v0_1.md`
- `work/goals/goal_run_eleventh_work_ledger_review.md`
- `work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md`
- `work/STATUS.md`

## What changed

- Replaced the `punk-gate` compile-only skeleton with a pure gate decision model.
- Added `GATE_DECISION_SCHEMA_VERSION`.
- Added gate decision refs and metadata:
  - `GateDecisionId`
  - `GateCreatedAt`
  - `GateContractRef`
  - `GateRunReceiptRef`
  - `GateEvalRef`
  - `GateEventRef`
  - `GateBoundaryNote`
- Added `GateDecisionOutcome`:
  - `accepted`
  - `rejected`
  - `needs_work`
  - `blocked`
  - `deferred`
- Added `GateDecision` with required contract refs, run receipt refs, created-at, boundary notes, and optional eval/event refs.
- Added `GateDecisionBoundary` to keep authority and side-effect boundaries explicit.
- Added tests for:
  - evidence refs staying referenced, not absorbed;
  - required refs and boundary notes;
  - accepting decision as final authority but not acceptance without proof;
  - non-accepting decisions not becoming acceptance;
  - no runtime/proofpack/CLI side effects;
  - empty ref rejection.
- Added the eleventh advisory Work Ledger Review as the next ready goal.

## Authority boundary

The gate decision kernel models final decision authority as data.

It still does not:

- write `.punk/decisions`;
- expose CLI behavior;
- create proofpacks;
- claim acceptance;
- require runtime storage;
- absorb receipt/eval/event evidence;
- add provider/model/agent adapters.

An accepting decision requires matching proof before acceptance can be claimed.

## Active CLI truth

No product CLI behavior changed.

Current implemented CLI surface remains:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

## Scope boundaries preserved

- no `.punk` runtime state written;
- no runtime storage implemented;
- no `Cargo.toml` or `Cargo.lock` changed;
- no CLI command added;
- no `punk init` implementation added;
- no schema file added;
- no proofpack writer added;
- no semantic assessor implementation added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Active-core gate kernel behavior changed, and the work ledger/report record the change; product docs and specs already covered the boundary."
  touched_surfaces:
    - crates/punk-gate/src/lib.rs
    - work/goals/goal_add_gate_decision_kernel_minimal_v0_1.md
    - work/goals/goal_run_eleventh_work_ledger_review.md
    - work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage added in `crates/punk-gate`."
```

## Checks run

- `cargo fmt --all --check` — PASS
- `git diff --check` — PASS
- `python3 scripts/check_research_gate.py` — PASS
- `python3 scripts/check_work_ledger.py` — PASS
- `scripts/check.sh docs-governance --files crates/punk-gate/src/lib.rs work/STATUS.md work/goals/goal_add_gate_decision_kernel_minimal_v0_1.md work/goals/goal_run_eleventh_work_ledger_review.md work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md --report work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md` — PASS
- `cargo check --workspace` — PASS
- `cargo test --workspace` — PASS
- `cargo run -p punk-cli -- eval run smoke --format json` — PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` — PASS (no repo-tracked artifact leaked the local absolute workspace path)

## Open follow-ups

- Run the eleventh advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack kernel, runtime storage, gate/eval integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If gate decision serialization/schema files are later needed, add them through a separate bounded schema goal.
