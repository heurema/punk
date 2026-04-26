---
id: report_2026_04_26_proofpack_writer_target_artifact_ref_policy_v0_1
goal_id: goal_define_proofpack_writer_target_artifact_ref_policy_v0_1
actor: vitaly
created_at: 2026-04-26
kind: execution
---

## Summary

Defined proofpack writer target artifact ref policy v0.1 as docs/spec-only boundary.

What changed:

- added `evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md`;
- selected `(proofpack_id, manifest_self_digest)` as the v0.1 canonical target artifact identity;
- rejected `proofpack_id` alone and `manifest_self_digest` alone as full target artifact identity;
- recorded `proofpack:<proofpack_id>@<manifest_self_digest>` as a recommended logical display ref vocabulary for future implementations;
- kept target artifact refs separate from canonical artifact bytes, target path refs, storage root refs, indexes, `latest` pointers, CLI output, service mirrors, and executor-local claims;
- defined idempotency/conflict expectations for same id/same digest and same id/different digest;
- documented future implementation requirements without activating writer behavior.

No Rust code, `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` was added.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded docs/spec policy definition derived from repo-tracked proofpack writer file IO, canonical artifact layout, canonical artifact model, and crate-status artifacts.
No external research was needed.
Decision:
Proceed.

Research refs used:

- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md`
- `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md`
- `evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md`
- `evals/specs/proofpack-writer-file-io-boundary.v0.1.md`
- `work/reports/2026-04-26-proofpack-writer-canonical-artifact-model-v0-1.md`
- `work/reports/2026-04-26-forty-seventh-work-ledger-review.md`

## Decision

The selected v0.1 target artifact identity is:

```text
(proofpack_id, manifest_self_digest)
```

`proofpack_id` alone is not sufficient because it does not identify exact canonical bytes.

`manifest_self_digest` alone is not sufficient because it loses explicit proofpack object identity and inspectable correlation.

A future implementation may render the selected identity as a logical display ref:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

That string is a logical ref, not a filesystem path.

## Acceptance Evidence

- `evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md` exists.
- The policy separates proofpack id, manifest self-digest, canonical artifact identity, target artifact ref, target path ref, storage root ref, indexes, `latest` pointers, CLI output, and service mirrors.
- The policy states target artifact identity must use both proofpack id and manifest self-digest.
- The policy rejects hidden authority from host paths, current working directory, chat state, executor-local memory, index views, `latest` pointers, service mirrors, provider/model/agent identity, and executor claims.
- The policy defines evidence versus authority boundaries and future implementation requirements without writing files or creating acceptance claims.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

This change did not:

- change Rust crates;
- write `.punk/proofs`;
- touch the filesystem for proofpack writes;
- canonicalize or inspect host filesystem paths;
- add schema files;
- add CLI commands;
- implement proofpack file writing;
- implement proofpack referenced-ref verification integration;
- implement gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, or agent integrations;
- implement `punk init`.

## Changed Files

- `evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_proofpack_writer_target_artifact_ref_policy_v0_1.md`
- `work/goals/goal_run_forty_eighth_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Docs/spec-only proofpack writer target artifact ref policy was added; work-ledger artifacts were updated."
  touched_surfaces:
    - evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_proofpack_writer_target_artifact_ref_policy_v0_1.md
    - work/goals/goal_run_forty_eighth_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test --workspace
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md work/STATUS.md work/goals/goal_define_proofpack_writer_target_artifact_ref_policy_v0_1.md work/goals/goal_run_forty_eighth_work_ledger_review.md work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md --report work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS
