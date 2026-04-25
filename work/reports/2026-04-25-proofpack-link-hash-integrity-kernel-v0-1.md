---
id: report_2026_04_25_proofpack_link_hash_integrity_kernel_v0_1
goal_id: goal_add_proofpack_link_hash_integrity_kernel_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add side-effect-free structural proofpack link/hash integrity helpers so Punk can inspect whether declared proofpack refs have matching artifact digest entries before any writer, runtime storage, CLI, or schema work.

## Research Gate

Classification: R0
Required: no
Rationale:
This implements already-defined proofpack and proof-before-acceptance boundaries in the existing Rust proof crate. No new architecture decision or external research was required.

Research refs:

- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md`
- `work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md`
- `work/reports/2026-04-25-thirteenth-work-ledger-review.md`

Decision:
Proceed with structural proofpack ref/digest integrity helpers only.

## Changed files

- `crates/punk-proof/src/lib.rs`
- `work/goals/goal_add_proofpack_link_hash_integrity_kernel_v0_1.md`
- `work/goals/goal_run_fourteenth_work_ledger_review.md`
- `work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `ProofArtifactDigestRequirement` to represent a required digest by artifact kind and artifact ref.
- Added `ProofpackIntegrityReport` with required digest refs and missing digest refs.
- Added `ProofArtifactDigest::satisfies_requirement`.
- Added proofpack helpers for:
  - `required_artifact_digest_refs`;
  - `has_artifact_digest_for`;
  - `link_hash_integrity_report`;
  - `has_complete_link_hash_integrity`;
  - `is_matching_proof_ready_for_acceptance`.
- Required integrity now covers:
  - gate decision ref;
  - contract refs;
  - run receipt refs;
  - optional eval refs when present;
  - optional event refs when present;
  - optional output artifact refs when present.
- Added boundary flags documenting that the kernel checks structural link/hash integrity but does not compute or normalize hashes.
- Added Rust unit coverage for complete, incomplete, minimal, optional-ref, wrong-kind/ref, and non-normalized hash cases.
- Added the fourteenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

The proofpack kernel still models post-gate provenance as data.

It still does not:

- write `.punk/proofs`;
- expose CLI behavior;
- write gate decisions;
- claim acceptance;
- require runtime storage;
- implement a proofpack writer;
- compute hashes;
- normalize hashes;
- absorb receipt/eval/event/output evidence;
- add provider/model/agent adapters.

Proofpack remains a provenance bundle.
Gate decision remains the final decision authority.
Missing required proofpack digests remain visible and block matching proof readiness signals.

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
    - work/goals/goal_add_proofpack_link_hash_integrity_kernel_v0_1.md
    - work/goals/goal_run_fourteenth_work_ledger_review.md
    - work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - "Rust unit coverage added in `crates/punk-proof`."
```

## Checks run

- `cargo test -p punk-proof` - PASS.
- `cargo fmt --all --check` - PASS.
- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-proof/src/lib.rs work/STATUS.md work/goals/goal_add_proofpack_link_hash_integrity_kernel_v0_1.md work/goals/goal_run_fourteenth_work_ledger_review.md work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md --report work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.

## Open follow-ups

- Run the fourteenth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack writer, runtime storage, gate/eval/proof orchestration, smoke eval updates, semantic assessor integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
- If proofpack serialization/schema files, writer behavior, or hash-normalization rules are later needed, add them through separate bounded goals.
