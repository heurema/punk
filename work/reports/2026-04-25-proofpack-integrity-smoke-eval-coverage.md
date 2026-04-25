---
id: report_2026_04_25_proofpack_integrity_smoke_eval_coverage
goal_id: goal_add_proofpack_integrity_smoke_eval_coverage
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add deterministic smoke eval coverage for proofpack link/hash integrity readiness using the existing side-effect-free proofpack kernel.

## Research Gate

Classification: R0
Required: no
Rationale:
This adds deterministic eval coverage over already-defined and implemented side-effect-free proofpack integrity helpers. No new architecture decision or external research was required.

Research refs:

- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md`
- `work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md`
- `work/reports/2026-04-25-fourteenth-work-ledger-review.md`

Decision:
Proceed with local deterministic smoke eval coverage only.

## Changed files

- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_add_proofpack_integrity_smoke_eval_coverage.md`
- `work/goals/goal_run_fifteenth_work_ledger_review.md`
- `work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md`
- `work/STATUS.md`

## What changed

- Extended the smoke suite from 17 to 19 deterministic cases.
- Added smoke cases for:
  - complete declared proofpack digest links satisfying matching proof readiness;
  - missing required proofpack digest links staying visible and blocking matching proof readiness.
- Updated the acceptance-chain smoke case to use proof readiness instead of only abstract proofpack ref matching.
- Kept smoke output as local assessment only.
- Added the fifteenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

The smoke eval remains local assessment only.

It still does not:

- write `.punk/evals`;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- compute hashes;
- normalize hashes;
- require runtime storage;
- add proofpack writer behavior;
- add gate/eval/proof orchestration;
- expose new CLI behavior;
- add provider/model/agent adapters.

Positive acceptance remains unavailable unless an accepting authority and matching proof readiness are both present.

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
    - crates/punk-eval/src/lib.rs
    - work/goals/goal_add_proofpack_integrity_smoke_eval_coverage.md
    - work/goals/goal_run_fifteenth_work_ledger_review.md
    - work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage updated in `crates/punk-eval`."
```

## Checks run

- `cargo test -p punk-eval` - PASS.
- `cargo fmt --all --check` - PASS.
- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_proofpack_integrity_smoke_eval_coverage.md work/goals/goal_run_fifteenth_work_ledger_review.md work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md --report work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.

## Open follow-ups

- Run the fifteenth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack writer, runtime storage, gate/eval/proof orchestration, schema/hash work, semantic assessor integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If proofpack serialization/schema files, writer behavior, or hash-normalization rules are later needed, add them through separate bounded goals.
