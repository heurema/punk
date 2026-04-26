---
id: report_2026_04_26_proofpack_writer_storage_schema_boundary_v0_1
goal_id: goal_define_proofpack_writer_storage_schema_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Defined proofpack writer storage/schema boundary v0.1 as docs/spec only.

The new boundary explains how a future proofpack writer may persist proofpack artifacts without activating storage/runtime authority:

- canonical proofpack artifacts should be append-only;
- manifest bytes and manifest self-digest metadata remain separate to avoid recursive self-reference;
- wrappers, indexes, and `latest` pointers are not canonical truth;
- schema/file-layout choices remain future explicit goals;
- missing, stale, or inconsistent views degrade inspect UX but do not change proof truth;
- setup neutrality is preserved and no IDE, CLI ritual, local runtime state, provider, model, skill, adapter, automation, or `punk init` is required.

No Rust code, `.punk/` runtime state, `.punk/proofs` directory, schema files, CLI behavior, proofpack writer behavior, proofpack referenced-ref verification integration implementation, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` were added.

## Research Gate

Classification: R1
Required: yes
Rationale:
This was a bounded docs/spec boundary derived from repo-tracked proofpack, project memory, storage, and hash-policy artifacts. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-writer-preparation-boundary.v0.1.md`
- `evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `crates/punk-proof/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md`
- `work/reports/2026-04-26-thirty-sixth-work-ledger-review.md`

## Changed Files

- `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_proofpack_writer_storage_schema_boundary_v0_1.md`
- `work/goals/goal_run_thirty_seventh_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-writer-storage-schema-boundary-v0-1.md`

## What changed

- Added a proofpack writer storage/schema docs/spec boundary.
- Defined future artifact classes: canonical proofpack artifact, manifest bytes, manifest self-digest metadata, wrapper metadata, indexes/views, and mutable `latest` pointer.
- Defined storage target, schema/file-layout, append-only overwrite, hidden-source-of-truth, failure, privacy, retention, and setup-neutral boundaries.
- Recorded that `.punk/proofs`, schema files, CLI behavior, proofpack writing, gate decisions, and acceptance claims remain inactive.
- Added the thirty-seventh advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI/`.punk` authority changed.

This did not:

- change Rust code;
- add dependencies;
- write `.punk/` state;
- create `.punk/proofs`;
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
  reason: "Added a docs/spec boundary for proofpack writer storage/schema semantics without promoting proofpack writer, runtime storage, schema files, CLI, gate, or acceptance scope."
  touched_surfaces:
    - evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_proofpack_writer_storage_schema_boundary_v0_1.md
    - work/goals/goal_run_thirty_seventh_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-writer-storage-schema-boundary-v0-1.md
  required_updates:
    - evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md
    - work/reports/2026-04-26-proofpack-writer-storage-schema-boundary-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md work/STATUS.md work/goals/goal_define_proofpack_writer_storage_schema_boundary_v0_1.md work/goals/goal_run_thirty_seventh_work_ledger_review.md work/reports/2026-04-26-proofpack-writer-storage-schema-boundary-v0-1.md --report work/reports/2026-04-26-proofpack-writer-storage-schema-boundary-v0-1.md` - PASS with 0 failures and 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the thirty-seventh advisory Work Ledger Review.
- Decide whether the next branch should be proofpack writer implementation preparation, `.punk/proofs` activation, schema files, proofpack referenced-ref verification integration implementation, additional smoke eval/docs guardrails, or another active-core setup step.
- Keep proofpack writer behavior, `.punk/` storage, runtime gate/proof behavior, schemas, CLI expansion, adapters, automation, provider/model runners, and `punk init` deferred until separately selected.
