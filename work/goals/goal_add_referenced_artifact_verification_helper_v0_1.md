---
id: goal_add_referenced_artifact_verification_helper_v0_1
title: "Add referenced artifact verification helper v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: 2026-04-26
completed_at: 2026-04-26
blocked_by: []
scope:
  include:
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "`punk-core` exposes a side-effect-bounded referenced artifact verification helper v0.1 that compares an expected canonical digest with the observed digest for one explicit regular file under one explicit repo root and validated repo-relative ref."
  - "The helper returns structured evidence-only outcomes for verified, digest mismatch, missing, directory/non-regular file, symlink, unreadable/read-denied/read-error, and root/ref/file boundary failures, without claiming gate or acceptance authority."
  - "Invalid raw refs and invalid raw digests remain explicit through existing typed constructors or validation helpers; the new helper does not accept raw unvalidated strings as hidden authority."
  - "`punk-eval` smoke coverage includes referenced artifact verification helper behavior as local assessment only."
  - "No proofpack writer behavior, proofpack writer hash-policy integration, `.punk/` runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` are added."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-eval/src/lib.rs"
  - "work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md"
  - "work/reports/2026-04-26-thirty-second-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded implementation derived from repo-tracked hash/file-IO/verification boundaries and current `punk-core` helpers; no external research is required unless an implementation conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md"
    - "work/reports/2026-04-26-thirty-second-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md"
  rationale: "The selected implementation should update `punk-core`, smoke eval coverage, work status, and a work report while preserving runtime/proof/CLI boundaries."
---

## Context

Referenced artifact verification boundary v0.1 is defined as docs/spec only.

`punk-core` already has:

- canonical artifact digest validation;
- repo-relative artifact ref validation;
- exact-byte digest computation;
- a narrow file IO artifact hashing helper for one explicit regular file.

The next bounded implementation can compare an expected canonical digest against an observed digest from the existing file IO helper and return evidence-only verification outcomes.

## Notes

Do not add CLI commands.
Do not write `.punk` state.
Do not add schema files.
Do not implement proofpack writer behavior.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
Keep setup-neutral behavior: no required IDE, CLI ritual, model, provider, prompt, skill, or local runtime setup.


## Outcome

Completed referenced artifact verification helper v0.1.

`punk-core` now exposes evidence-only referenced artifact verification over explicit typed repo root, repo-relative artifact ref, and expected canonical digest inputs. The helper compares the expected digest with the observed digest from one explicit regular file and returns structured outcomes without proofpack writer, runtime, CLI, gate decision, or acceptance authority.

`punk-eval` smoke coverage now includes referenced artifact verification helper behavior as local assessment only.

No `.punk/` runtime state, schema files, CLI behavior, proofpack writer behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` were added.
