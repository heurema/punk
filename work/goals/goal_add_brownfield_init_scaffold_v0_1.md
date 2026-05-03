---
id: goal_add_brownfield_init_scaffold_v0_1
title: "Add brownfield init scaffold v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P1
authority: canonical
created_at: 2026-05-03
updated_at: 2026-05-03
selected_at: 2026-05-03
started_at: 2026-05-03
completed_at: 2026-05-03
blocked_by: []
scope:
  include:
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-cli/src/main.rs"
    - "crates/punk-eval/src/lib.rs"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/NORTH-STAR.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/FLOW.md"
    - "docs/product/DOGFOODING.md"
    - "scripts/check_docs_governance.py"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_brownfield_init_scaffold_v0_1.md"
    - "work/goals/goal_verify_brownfield_init_scaffold_v0_1.md"
    - "work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md"
  exclude:
    - ".punk/**"
    - "schemas/**"
    - "crates/punk-contract/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
acceptance:
  - "`punk init <project-id> --mode brownfield` creates a compact `.punk/memory/` brownfield entry scaffold."
  - "Greenfield `punk init <project-id>` default behavior remains unchanged."
  - "Brownfield scaffold records `entry_mode = brownfield`, `reconstruction_status = not_started`, and `authority = advisory_candidates_only`."
  - "Brownfield scaffold creates empty advisory reconstruction placeholders and no reconstructed claims."
  - "Brownfield init does not scan the repository, generate summaries, generate contracts/specs, write runtime state, create gate/proof artifacts, or invoke Writer behavior."
  - "Atomic preflight and privacy-safe `target_root: .` output apply to brownfield mode."
knowledge_refs:
  - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
  - "work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md"
  - "docs/product/START-HERE.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local product slice selected by the maintainer from existing advisory research and current init evidence; it adds only a scaffold, not reconstruction."
  research_refs:
    - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
    - "work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md"
    - "docs/product/PROJECT-MEMORY.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/NORTH-STAR.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/FLOW.md"
    - "docs/product/DOGFOODING.md"
    - "scripts/check_docs_governance.py"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "This changes the active init CLI surface and current product docs while preserving reconstruction/runtime boundaries."
---

## Context

Greenfield init is already compact, atomic, privacy-safe, and current-directory explicit.

The next selected Punk slice is a brownfield entry scaffold, not brownfield reconstruction.

## Intent

Let an existing project start Punk memory with an explicit advisory reconstruction workspace.

## Non-scope

Do not implement repo scanning, file inventory generation, language detection, docs extraction, source graph, AI summaries, LLM calls, issue/PR/commit mining, agent transcript import, contract generation, spec generation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, gate/proof runtime, Writer, runtime `.punk` storage, brownfield claim acceptance, or grayfield reconciliation.

## Outcome

`punk init <project-id> --mode brownfield` now creates a compact `.punk/memory/` brownfield entry scaffold with empty advisory reconstruction placeholders and not-started reconstruction state.

Greenfield default behavior remains `punk init <project-id>`.
