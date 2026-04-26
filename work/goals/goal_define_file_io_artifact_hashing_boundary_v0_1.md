---
id: goal_define_file_io_artifact_hashing_boundary_v0_1
title: "Define file IO artifact hashing boundary v0.1"
status: done
owner: "vitaly"
module: "proof"
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
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "A docs/spec boundary defines what future file IO artifact hashing may mean before any implementation."
  - "The boundary distinguishes exact-byte in-memory hashing from future file reads, referenced artifact byte verification, proofpack writer behavior, runtime storage, schemas, CLI behavior, gate decisions, and acceptance claims."
  - "The boundary keeps repo-relative refs, missing files, privacy, symlinks, directories, generated artifacts, and user setup neutrality explicit enough to prevent silent runtime scope expansion."
  - "No Rust code, schema file, CLI command, runtime storage, `.punk/` state, proofpack writer, file IO hashing implementation, referenced artifact byte verification implementation, provider/model/agent adapter, automation, or `punk init` is added."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md"
  - "work/reports/2026-04-26-twenty-eighth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a docs/spec boundary over repo-tracked hash/proofpack evidence before any file IO hashing implementation; no external research is required unless a runtime or security boundary conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md"
    - "work/reports/2026-04-26-twenty-eighth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md"
  rationale: "The selected goal should define a docs/spec boundary before any runtime file IO hashing or verification implementation."
---

## Context

Punk has artifact hash policy validation, exact-byte in-memory hash computation, proofpack manifest rendering, and proofpack manifest self-digest computation.

Punk still does not read artifact files, verify referenced artifact bytes, write proofpacks, write `.punk/` state, expose hash CLI behavior, or create acceptance claims.

Before any file IO artifact hashing helper, proofpack writer hash integration, or referenced artifact byte verification is selected, define a boundary that keeps file reads, repo roots, privacy, missing files, symlinks, directories, generated artifacts, and user setup neutrality explicit.

## Notes

Do not change Rust code.
Do not add schema files.
Do not add CLI commands.
Do not write `.punk` state.
Do not implement file IO hashing.
Do not verify referenced artifact bytes.
Do not implement proofpack writer behavior.
Do not imply gate or acceptance authority.


## Outcome

Defined `evals/specs/file-io-artifact-hashing-boundary.v0.1.md` as a docs/spec boundary before any file IO artifact hashing implementation.

The boundary distinguishes caller-provided exact-byte hashing, proofpack manifest self-digest metadata, future artifact file reads, referenced artifact byte verification, proofpack writer behavior, runtime storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`.

The boundary keeps repo-relative refs, explicit repo roots, missing files, non-regular files, symlinks, directories, generated artifacts, privacy, and setup neutrality explicit.

No Rust code, schema file, CLI command, runtime storage, `.punk/` state, proofpack writer, file IO hashing implementation, referenced artifact byte verification implementation, provider/model/agent adapter, automation, or `punk init` was added.
