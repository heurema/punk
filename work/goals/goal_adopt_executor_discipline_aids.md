---
id: goal_adopt_executor_discipline_aids
title: "Adopt executor discipline aids as advisory artifacts"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: null
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-04-25-karpathy-skills-review.md"
    - "examples/executor-briefs/punk-default.md"
    - "examples/contracts/task-transformations.md"
    - "examples/punk-workflows/karpathy-style-failures.md"
    - "evals/specs/surgical-change-discipline.v0.1.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/DOGFOODING.md"
    - "work/goals/goal_adopt_executor_discipline_aids.md"
    - "work/reports/2026-04-25-executor-discipline-aids.md"
  exclude:
    - "crates/**"
    - "Cargo.toml"
    - "Cargo.lock"
    - ".github/**"
    - ".punk/**"
    - "site/**"
    - "scripts/**"
    - "brand/**"
acceptance:
  - "Karpathy-style executor discipline is adopted as Punk-native advisory docs, examples, and eval-policy artifacts only."
  - "No runtime, code, schema, CLI, provider adapter, model runner, prompt manager, or skill system change is introduced."
  - "Executor aids are explicitly advisory and non-authoritative."
  - "Executor claims remain non-proof, semantic assessors remain advisory, and gate remains final decision authority."
  - "The patch preserves active-core vs future/incubating boundaries."
knowledge_refs:
  - "knowledge/research/2026-04-25-karpathy-skills-review.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-executor-discipline-aids.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "The change adopts external prompt/skill prior art into docs/examples/eval-policy only, touching active-core/future boundary wording without runtime behavior."
  research_refs:
    - "knowledge/research/2026-04-25-karpathy-skills-review.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
  external_research_refs:
    - "https://github.com/forrestchang/andrej-karpathy-skills"
    - "https://docs.anthropic.com/en/docs/claude-code/skills"
    - "https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills"
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/DOGFOODING.md"
  rationale: "Adds advisory executor-aid examples and eval-policy guidance without changing runtime behavior or canonical authority surfaces."
---

## Context

Punk can benefit from executor discipline patterns without adopting an executor-specific skill, prompt package, or authority model.

This goal adopts the useful ideas from `forrestchang/andrej-karpathy-skills` as Punk-native advisory artifacts:

- assumptions before edits;
- simplicity first;
- surgical changes;
- verifiable goals.

## Notes

This goal is docs/examples/eval-policy only.

Out of scope:

- `punk brief` or any new CLI command;
- runtime enforcement;
- schema fields;
- provider adapters;
- model runner code;
- prompt or skill management;
- public site copy;
- `work/STATUS.md` changes under this strict allowed-path contract.
