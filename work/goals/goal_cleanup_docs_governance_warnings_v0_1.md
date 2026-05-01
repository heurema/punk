---
id: goal_cleanup_docs_governance_warnings_v0_1
title: "Cleanup docs-governance warnings v0.1"
status: done
owner: "vitaly"
module: "process"
priority: P2
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/**"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/GLOSSARY.md"
  exclude:
    - ".punk/**"
    - "schemas/**"
    - "crates/**"
acceptance:
  - "The accepted/deferred docs-governance warnings are reviewed and either fixed surgically or explicitly deferred with rationale."
  - "No product behavior, runtime behavior, CLI, storage, Writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is added."
  - "The work ledger remains consistent and selects the next bounded non-runtime goal."
knowledge_refs:
  - "work/STATUS.md"
  - "work/reports/2026-05-01-close-contract-core-checkpoint-series-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is repo-local docs/process cleanup over existing docs-governance warnings; external research is not needed unless product meaning changes."
  research_refs:
    - "work/STATUS.md"
    - "work/reports/2026-05-01-close-contract-core-checkpoint-series-v0-1.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/PROJECT-MEMORY.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-governance-cleanup
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The checkpoint series is closed; existing accepted/deferred docs-governance warnings are the next safe non-runtime cleanup target."
---

## Context

The contract-core checkpoint review series is closed. Existing docs-governance warnings remain accepted/deferred.

## Intent

Review and clean up docs-governance warnings surgically, or explicitly defer them with rationale.

## Outcome

The four known docs-governance warnings were fixed with minimal wording changes:

- `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`.
- `docs/product/CRATE-STATUS.md`: `Current CLI surface`.
- `docs/product/DOCUMENTATION-MAP.md`: `Research notes`.
- `docs/product/PROJECT-MEMORY.md`: `Project coherence`.

No product behavior, runtime behavior, CLI behavior, storage, Writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or `punk init` behavior was added.

The next selected goal is `work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md`.

## Non-scope

Do not implement runtime behavior.

Do not implement Writer.

Do not add CLI behavior, `.punk` runtime storage, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or `punk init`.
