---
id: goal_harden_greenfield_init_atomic_and_privacy_v0_1
title: "Harden greenfield init atomicity and privacy v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-cli/src/main.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/goal_harden_greenfield_init_atomic_and_privacy_v0_1.md"
    - "work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md"
    - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
  exclude:
    - ".punk/**"
    - ".punk/runtime/**"
    - ".punk/cache/**"
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
acceptance:
  - "`punk init <project-id>` preflights the full greenfield scaffold before writing scaffold files or directories."
  - "If any blocking conflict exists, init returns blocked without writing missing planned artifacts."
  - "Existing modified scaffold files are preserved."
  - "Normal human output does not print an absolute local `target_root`; it renders `target_root: .`."
  - "Tests cover conflict no-op, failed preflight partial-scaffold prevention, and target-root redaction."
  - "No brownfield, grayfield, runtime storage, contracts, gate/proof runtime, Writer, agents/adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is added."
knowledge_refs:
  - "work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md"
  - "work/reports/2026-05-01-roadmap-status-reconciliation-after-greenfield-init-v0-1.md"
  - "docs/product/START-HERE.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local hardening slice over already active greenfield init behavior, selected from maintainer-provided audit input and existing Punk status docs; no new external research is needed."
  research_refs:
    - "work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md"
    - "work/reports/2026-05-01-roadmap-status-reconciliation-after-greenfield-init-v0-1.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The active init implementation changes operational safety and human output privacy."
---

## Context

`punk init <project-id>` is active as a greenfield-only Dogfooding Level 0 compact `.punk/memory/` scaffold.

The maintainer-provided audit narrowed the next Punk work to operational hardening of that first setup surface.

## Intent

Make init fail closed before any scaffold writes when a conflict exists, and prevent normal human output from leaking absolute local paths.

## Non-scope

Do not implement brownfield init, grayfield reconciliation, repo scan, AI summaries, contract generation, gate/proof runtime, Writer, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, runtime `.punk` storage, or generated docs.

## Outcome

`punk init <project-id>` now preflights every planned scaffold entry before creating directories or files.

When a blocking conflict is detected, the report returns `result: blocked`, preserves existing modified files, marks missing entries as `planned`, and writes no missing scaffold artifacts.

Normal human output now renders `target_root: .` instead of an absolute host path.

Selected `work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md` next for docs/help polish and the lower-priority flow inspect preview wording cleanup.
