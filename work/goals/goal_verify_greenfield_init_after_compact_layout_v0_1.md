---
id: goal_verify_greenfield_init_after_compact_layout_v0_1
title: "Verify greenfield init after compact layout v0.1"
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
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "`punk init <project-id>` compact `.punk/memory/` behavior is verified as the current user-facing greenfield milestone through smoke/manual checks and docs status inspection."
  - "Any roadmap/status drift found during verification is either fixed surgically or routed to a future goal."
  - "No product behavior, runtime behavior, CLI expansion, storage, Writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, brownfield init, grayfield reconcile, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is added."
knowledge_refs:
  - "work/reports/2026-05-01-greenfield-init-compact-memory-layout-v0-1.md"
  - "work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is repo-local product verification over already implemented greenfield init behavior; external research is not needed unless product meaning changes."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/reports/2026-05-01-greenfield-init-compact-memory-layout-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-governance-cleanup
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The compact greenfield init milestone has just changed user-facing setup behavior; one narrow verification pass should confirm docs and smoke/manual evidence before new architecture work."
---

## Context

Greenfield init now creates compact tracked `.punk/memory/` durable memory for user projects, while runtime storage and broader entry modes remain inactive.

## Intent

Verify the compact init milestone as a product surface before selecting new architecture or runtime work.

## Outcome

Verification passed without implementation changes.

Fresh temp init created only compact tracked `.punk/memory/` durable memory plus `.punk/README.md` and `.punk/project.toml`, left root-level Punk directories and runtime directories absent, and kept `.punk/memory/**` visible to git.

Project id validation rejected uppercase, underscore, and empty ids. Re-running init on matching generated files is safe idempotent and reports `already_initialized`; modified existing memory is treated as conflict and preserved.

No network, repo scanning, generated contracts, gate/proof runtime, Writer, agents, adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was activated.

The next selected goal is `work/goals/goal_publish_greenfield_init_checkpoint_status_v0_1.md`.

## Non-scope

Do not implement brownfield init, grayfield reconcile, runtime storage, Writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or CLI expansion.
