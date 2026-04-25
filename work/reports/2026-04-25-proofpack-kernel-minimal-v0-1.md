---
id: report_2026_04_25_proofpack_kernel_minimal_v0_1
goal_id: goal_add_proofpack_kernel_minimal_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add a minimal side-effect-free proofpack kernel v0.1 so Punk can model post-gate provenance without activating `.punk/proofs`, proofpack writer behavior, CLI, runtime storage, or acceptance claims.

## Research Gate

Classification: R0
Required: no
Rationale:
This implements already-defined proofpack and proof-before-acceptance boundaries in the existing Rust proof crate. No new architecture decision or external research was required.

Research refs:

- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md`
- `work/reports/2026-04-25-eleventh-work-ledger-review.md`

Decision:
Proceed with a side-effect-free proofpack kernel implementation only.

## Changed files

- `crates/punk-proof/src/lib.rs`
- `work/goals/goal_add_proofpack_kernel_minimal_v0_1.md`
- `work/goals/goal_run_twelfth_work_ledger_review.md`
- `work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md`
- `work/STATUS.md`

## What changed

- Replaced the `punk-proof` compile-only skeleton with a pure proofpack provenance model.
- Added `PROOFPACK_SCHEMA_VERSION`.
- Added proofpack refs and metadata:
  - `ProofpackId`
  - `ProofCreatedAt`
  - `ProofGateDecisionRef`
  - `ProofContractRef`
  - `ProofRunReceiptRef`
  - `ProofEvalRef`
  - `ProofEventRef`
  - `ProofOutputArtifactRef`
  - `ProofBoundaryNote`
- Added artifact hash metadata:
  - `ProofArtifactKind`
  - `ProofArtifactRef`
  - `ProofArtifactHash`
  - `ProofArtifactDigest`
- Added `Proofpack` with required gate decision ref, contract refs, run receipt refs, created-at, boundary notes, and optional eval/event/output refs and artifact digests.
- Added `ProofpackBoundary` to keep provenance, authority, and side-effect boundaries explicit.
- Added positive acceptance precondition helper that requires both an accepting gate decision and a matching proofpack.
- Added tests for:
  - evidence refs and hashes staying referenced, not absorbed;
  - required refs and boundary notes;
  - post-gate provenance without final decision authority;
  - no runtime writer, CLI, or acceptance side effects;
  - positive acceptance requiring both accepting gate decision and matching proofpack;
  - stable artifact-kind vocabulary;
  - empty ref rejection.
- Added the twelfth advisory Work Ledger Review as the next ready goal.

## Authority boundary

The proofpack kernel models post-gate provenance as data.

It still does not:

- write `.punk/proofs`;
- expose CLI behavior;
- write gate decisions;
- claim acceptance;
- require runtime storage;
- absorb receipt/eval/event/output evidence;
- add provider/model/agent adapters.

Proofpack remains a provenance bundle.
Gate decision remains the final decision authority.
Positive acceptance requires both an accepting gate decision and a matching proofpack.

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
- no gate/eval/proof orchestration added;
- no semantic assessor implementation added;
- no provider/model/agent adapter added;
- no automation added.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Active-core proofpack kernel behavior changed, and the work ledger/report record the change; product docs and specs already covered the boundary."
  touched_surfaces:
    - crates/punk-proof/src/lib.rs
    - work/goals/goal_add_proofpack_kernel_minimal_v0_1.md
    - work/goals/goal_run_twelfth_work_ledger_review.md
    - work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage added in `crates/punk-proof`."
```

## Checks run

- `cargo fmt --all --check` — PASS
- `git diff --check` — PASS
- `python3 scripts/check_research_gate.py` — PASS
- `python3 scripts/check_work_ledger.py` — PASS
- `scripts/check.sh docs-governance --files crates/punk-proof/src/lib.rs work/STATUS.md work/goals/goal_add_proofpack_kernel_minimal_v0_1.md work/goals/goal_run_twelfth_work_ledger_review.md work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md --report work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md` — PASS
- `cargo check --workspace` — PASS
- `cargo test --workspace` — PASS
- `cargo run -p punk-cli -- eval run smoke --format json` — PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` — PASS (no repo-tracked artifact leaked the local absolute workspace path)

## Open follow-ups

- Run the twelfth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack writer, runtime storage, gate/eval/proof orchestration, semantic assessor integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If proofpack serialization/schema files or hash-normalization rules are later needed, add them through separate bounded goals.
