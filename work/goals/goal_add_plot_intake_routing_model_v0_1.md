---
id: goal_add_plot_intake_routing_model_v0_1
title: "Add Plot Intake routing model v0.1"
status: done
owner: "vitaly"
module: "plot-intake"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: 2026-06-08
completed_at: 2026-06-08
blocked_by: []
scope:
  include:
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "work/goals/goal_add_plot_intake_routing_model_v0_1.md"
    - "work/reports/2026-06-08-plot-intake-routing-model-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/punk-cli/**"
    - "crates/punk-module-host/**"
    - "crates/punk-mod-pubpunk/**"
    - "docs/adapters/**"
    - "publishing/**"
acceptance:
  - "Adds a side-effect-free Plot Intake routing model for slash-command-shaped request text."
  - "Routes `/punk ...` content requests to advisory `phase_route=plot`, `module_route=pubpunk`, and `next_handoff=pubpunk.contract_intake`."
  - "Treats `/pub ...` and `/dev ...` as non-authoritative route hints."
  - "Keeps plain text without a supported prefix uncaptured and empty/unclear requests clarification-required."
  - "Adds unit and smoke coverage without activating CLI behavior, harness adapters, `.punk` writes, module execution, provider calls, publishing, receipts, gate decisions, proofpacks, or acceptance claims."
knowledge_refs:
  - "docs/product/PLOT-INTAKE.md"
  - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-plot-intake-routing-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This user-directed scope override implements a side-effect-free model from repo-tracked Plot Intake docs/eval specs. External research is not needed because the slice does not add adapters, provider calls, runtime routing, or module execution."
  research_refs:
    - "docs/product/PLOT-INTAKE.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The slice adds side-effect-free Plot Intake routing model behavior and smoke coverage while updating current-status docs and work-ledger evidence."
---

## Context

The maintainer selected a user-directed override from the canonical Brownfield
decision checkpoint to test the `/punk ...` routing idea in code.

Repo-tracked Plot Intake docs already define a future harness slash-command
bridge and require routing to remain advisory, explainable, and
non-authoritative.

## Intent

Add the first pure routing model before any CLI, harness adapter, Module Host
runtime, or PubPunk module execution work.

## Outcome

Done with code-doc/work-ledger evidence.

## Non-Scope

Do not implement public CLI routing, harness slash-command files, Codex, Claude
Code, Gemini, `agy`, or other harness adapters, provider/model calls, `.punk` writes, Module Host
runtime, module loading, PubPunk module execution, draft writing, publishing,
browser/API calls, credential access, receipt writing, event writing, gate
decisions, proofpacks, or acceptance claims.
