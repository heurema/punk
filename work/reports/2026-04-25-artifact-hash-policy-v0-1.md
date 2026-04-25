---
id: report_2026_04_25_artifact_hash_policy_v0_1
goal_id: goal_define_artifact_hash_policy_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Define artifact hash policy v0.1 as a docs/spec-only boundary before active hash computation, hash normalization, proofpack writer, runtime storage, schema files, or gate/eval/proof orchestration.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a docs/spec policy task grounded in existing canonical docs, ADRs, and proofpack/proof-before-acceptance specs. No external research or Deep Research was needed because no new cryptographic architecture or implementation dependency was selected.

Research refs:

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `work/reports/2026-04-25-sixteenth-work-ledger-review.md`

Decision:
Proceed with a docs/spec-only artifact hash policy.

## Changed files

- `evals/specs/artifact-hash-policy.v0.1.md`
- `work/goals/goal_define_artifact_hash_policy_v0_1.md`
- `work/goals/goal_run_seventeenth_work_ledger_review.md`
- `work/reports/2026-04-25-artifact-hash-policy-v0-1.md`
- `work/STATUS.md`

## What changed

- Added `evals/specs/artifact-hash-policy.v0.1.md`.
- Defined canonical v0.1 digest identity as `sha256:<64 lowercase hex chars>`.
- Defined artifact ref policy for repo-relative refs and runtime id refs.
- Defined exact-byte hashing boundary and explicitly avoided semantic/Markdown/JSON/line-ending normalization claims.
- Defined missing, invalid, unsupported, ref-mismatch, unverified, and not-required digest states.
- Linked hash policy to receipts, evals, semantic assessments, gate decisions, proofpacks, and Project Memory without turning hashes into acceptance authority.
- Added deterministic future eval cases `AHP-001` through `AHP-010`.
- Added the seventeenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

This was a docs/spec-only policy update.

It did not:

- change Rust code;
- add a hashing dependency;
- compute hashes;
- normalize hashes in code;
- add schema files;
- add CLI behavior;
- write `.punk/` state;
- implement gate or proofpack writers;
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
  classification: architecture
  reason: "Added an advisory docs/spec artifact defining artifact hash policy before active hash implementation, proofpack writer, schema, or runtime storage work."
  touched_surfaces:
    - evals/specs/artifact-hash-policy.v0.1.md
    - work/goals/goal_define_artifact_hash_policy_v0_1.md
    - work/goals/goal_run_seventeenth_work_ledger_review.md
    - work/reports/2026-04-25-artifact-hash-policy-v0-1.md
    - work/STATUS.md
  required_updates:
    - evals/specs/artifact-hash-policy.v0.1.md
    - work/reports/2026-04-25-artifact-hash-policy-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files evals/specs/artifact-hash-policy.v0.1.md work/STATUS.md work/goals/goal_define_artifact_hash_policy_v0_1.md work/goals/goal_run_seventeenth_work_ledger_review.md work/reports/2026-04-25-artifact-hash-policy-v0-1.md --report work/reports/2026-04-25-artifact-hash-policy-v0-1.md` - PASS.
- `cargo test --workspace` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.

## Open follow-ups

- Run the seventeenth advisory Work Ledger Review.
- Decide whether to implement small side-effect-free hash format/ref validation helpers, add smoke eval coverage for hash policy cases, or keep deferring runtime writer/storage work.
- Keep proofpack writer, active hash computation, `.punk/` storage, schema files, gate/eval/proof orchestration, adapters, automation, and `punk init` deferred until separately selected.
