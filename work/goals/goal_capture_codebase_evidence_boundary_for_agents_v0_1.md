---
id: goal_capture_codebase_evidence_boundary_for_agents_v0_1
title: "Capture codebase evidence boundary for agents v0.1"
status: done
owner: "vitaly"
module: "adapters"
priority: P2
authority: canonical
created_at: 2026-05-04
updated_at: 2026-05-04
selected_at: 2026-05-04
started_at: 2026-05-04
completed_at: 2026-05-04
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-05-03-codebase-understanding-for-agents.md"
    - "evals/specs/repo-search-adapter-boundary.v0.1.md"
    - "evals/specs/codebase-context-pack-boundary.v0.1.md"
    - "docs/adapters/repo-search.md"
    - "work/STATUS.md"
    - "work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md"
    - "work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ARCHITECTURE.md"
    - "README.md"
acceptance:
  - "Adds an R2 draft/advisory research note for Codebase Evidence Boundary for Agents."
  - "Adds advisory eval/spec boundaries for repo-search and codebase context-pack evidence."
  - "Keeps repo-search parked/advisory/documentation-only."
  - "Adds no runtime, CLI, MCP, agent execution, repo scanning, vector, graph, gate writer, acceptance proof behavior, or new proofpack writer behavior."
knowledge_refs:
  - "knowledge/research/2026-05-03-codebase-understanding-for-agents.md"
  - "knowledge/research/2026-04-19-repo-search-adapter-options.md"
  - "knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md"
  - "docs/adapters/repo-search.md"
  - "evals/specs/repo-search-adapter-boundary.v0.1.md"
  - "evals/specs/codebase-context-pack-boundary.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "The task changes advisory research/eval direction for knowledge retrieval, repo-search adapter boundaries, and codebase context evidence, but does not implement runtime behavior."
  research_refs:
    - "knowledge/research/2026-05-03-codebase-understanding-for-agents.md"
    - "knowledge/research/2026-04-19-repo-search-adapter-options.md"
    - "knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "knowledge/research/2026-05-03-codebase-understanding-for-agents.md"
    - "evals/specs/repo-search-adapter-boundary.v0.1.md"
    - "evals/specs/codebase-context-pack-boundary.v0.1.md"
    - "docs/adapters/repo-search.md"
    - "work/STATUS.md"
    - "work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md"
  rationale: "The goal records advisory research and eval/spec boundaries for codebase evidence without activating repo-search, adapters, MCP, Dev module, agent execution, runtime storage, vector search, code graph runtime, gate writer, broader runtime proofpack writer orchestration, `.punk/proofs` writer behavior, or acceptance proof behavior."
---

## Context

The maintainer requested a bounded documentation and eval-spec patch for
Codebase Evidence Boundary for Agents.

The repo-search adapter is currently parked and documentation-only.

## Intent

Record the research conclusion that Punk should treat repository assistance for
agents as bounded local evidence with receipts and clause-indexed context, not
as agent-owned project truth.

## Non-scope

Do not implement CLI behavior, runtime storage, repo scanning, file walking,
MCP integration, provider adapters, agent execution, Dev module runtime, vector
DB, embedding index, code graph writer, Knowledge Vault runtime, Context
Compiler runtime, broader runtime proofpack writer orchestration,
`.punk/proofs` writer behavior, acceptance proof behavior, or gate decision
writer behavior.

## Outcome

Done in `work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md`.

The patch adds one R2 advisory research note, two advisory eval/spec boundary
documents, a minimal parked repo-search adapter doc update, and Level 0 work
evidence.

This patch adds no new proofpack writer behavior.

No runtime, CLI, MCP, agent execution, repo scanning, vector, graph, gate
writer, or acceptance proof behavior was activated.
