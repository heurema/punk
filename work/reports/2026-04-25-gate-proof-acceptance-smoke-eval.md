---
id: report_2026_04_25_gate_proof_acceptance_smoke_eval
goal_id: goal_add_gate_proof_acceptance_smoke_eval
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add deterministic smoke eval coverage for the gate/proof acceptance chain using the existing side-effect-free gate and proofpack kernels.

## Research Gate

Classification: R0
Required: no
Rationale:
This adds deterministic eval coverage over already-defined and implemented side-effect-free gate/proof kernels. No new architecture decision or external research was required.

Research refs:

- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md`
- `work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md`
- `work/reports/2026-04-25-twelfth-work-ledger-review.md`

Decision:
Proceed with local deterministic smoke eval coverage only.

## Changed files

- `Cargo.lock`
- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_add_gate_proof_acceptance_smoke_eval.md`
- `work/goals/goal_run_thirteenth_work_ledger_review.md`
- `work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md`
- `work/STATUS.md`

## What changed

- Added `punk-gate` and `punk-proof` as local path dependencies for `punk-eval`.
- Updated `Cargo.lock` for the new local path dependencies.
- Extended the smoke suite from 14 to 17 deterministic cases.
- Added smoke cases for:
  - gate authority requiring proof before acceptance;
  - proofpack staying post-gate provenance and not decision authority;
  - positive acceptance requiring both accepting authority and a matching proofpack.
- Updated smoke assessment wording to cover current contract, flow, receipt, event, gate, and proof kernels.
- Added a boundary note that gate/proof smoke cases remain local assessment and do not claim acceptance.
- Updated JSON/human output tests for the added cases.
- Added the thirteenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

The smoke eval remains local assessment only.

It still does not:

- write `.punk/evals`;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- require runtime storage;
- add proofpack writer behavior;
- add gate/eval/proof orchestration;
- expose new CLI behavior;
- add provider/model/agent adapters.

Positive acceptance remains unavailable unless an accepting gate decision and a matching proofpack are both present.

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
- no CLI command added;
- no `punk init` implementation added;
- no schema file added;
- no proofpack writer added;
- no gate/eval/proof orchestration added;
- no semantic assessor implementation added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Active-core eval behavior changed, and the work ledger/report record the change; product docs and specs already covered the boundary."
  touched_surfaces:
    - Cargo.lock
    - crates/punk-eval/Cargo.toml
    - crates/punk-eval/src/lib.rs
    - work/goals/goal_add_gate_proof_acceptance_smoke_eval.md
    - work/goals/goal_run_thirteenth_work_ledger_review.md
    - work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage updated in `crates/punk-eval`."
```

## Checks run

- `cargo fmt --all --check` — PASS
- `git diff --check` — PASS
- `python3 scripts/check_research_gate.py` — PASS
- `python3 scripts/check_work_ledger.py` — PASS
- `scripts/check.sh docs-governance --files Cargo.lock crates/punk-eval/Cargo.toml crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_gate_proof_acceptance_smoke_eval.md work/goals/goal_run_thirteenth_work_ledger_review.md work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md --report work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md` — PASS
- `cargo check --workspace` — PASS
- `cargo test --workspace` — PASS
- `cargo run -p punk-cli -- eval run smoke --format json` — PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` — PASS (no repo-tracked artifact leaked the local absolute workspace path)

## Open follow-ups

- Run the thirteenth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack writer, runtime storage, gate/eval/proof orchestration, semantic assessor integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If acceptance-chain schema fixtures are later needed, add them through a separate bounded goal.
