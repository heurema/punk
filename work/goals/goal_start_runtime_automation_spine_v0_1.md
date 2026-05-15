---
id: goal_start_runtime_automation_spine_v0_1
title: "Start Runtime Automation Spine v0.1"
status: done
owner: "vitaly"
module: "runtime"
priority: P1
authority: canonical
created_at: 2026-05-13
updated_at: 2026-05-13
selected_at: 2026-05-13
started_at: 2026-05-13
completed_at: 2026-05-13
blocked_by: []
scope:
  include:
    - "crates/punk-events/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-cli/src/main.rs"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/goals/goal_start_runtime_automation_spine_v0_1.md"
    - "work/reports/2026-05-13-runtime-automation-spine-v0-1.md"
  exclude:
    - ".github/**"
    - ".punk/**"
    - "crates/**/adapters/**"
    - "docs/adr/**"
    - "evals/**"
    - "publishing/**"
    - "scripts/**"
acceptance:
  - "The smallest safe local Runtime Automation Spine prerequisite is selected and implemented."
  - "A local-only event writer can append caller-provided flow event drafts to `.punk/events/flow.jsonl` under an explicit initialized project root."
  - "The writer requires an explicit absolute project root and `.punk/project.toml` marker."
  - "The writer rejects unsafe artifact refs and event path conflicts fail-closed."
  - "Event records remain evidence only and do not write gate decisions, proofpacks, acceptance claims, run receipts, or product truth."
  - "Smoke eval coverage proves the bounded local event writer behavior."
  - "Docs/status distinguish the narrow active event writer from broader parked runtime storage."
  - "No external side effects, PubPunk, CommunityPunk, provider adapters, GitHub API behavior, Telegram/Discord behavior, bot runtime, autonomous PR creation, live publishing, raw transcript storage, DAO, token, funding, or cloud sync is activated."
knowledge_refs:
  - "README.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "CONTRIBUTING.md"
  - "work/STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-13-runtime-automation-spine-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This change promotes the first local runtime event-storage writer slice using existing product laws, architecture, roadmap, and crate-status constraints; external research is not needed because the slice is local-only, stdlib-based, and avoids broader persistence, replay, external side effects, and authority claims."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/RESEARCH-GATE.md"
    - "CONTRIBUTING.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The first local runtime event writer slice changes current crate behavior and smoke coverage, so current surface docs and work-ledger evidence were updated."
---

## Context

Manual Community Lab publication and digest work is no longer the product path.

The product path is zero-manual Punk, but external actions must wait until the local runtime can preserve trustworthy state, events, receipts, decisions, proof refs, and side-effect receipts.

## Selected Slice

Implemented the smallest active-core prerequisite: a bounded local event writer in `punk-events`.

The writer appends caller-provided flow event drafts to `.punk/events/flow.jsonl` under an explicit initialized project root.

## Boundary

This is local event evidence only.

It does not add persisted flow state, transition commands, receipts, gate decisions, proofpacks, acceptance claims, publication automation, provider adapters, bots, GitHub API calls, or autonomous PR behavior.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with code, smoke coverage, docs/status, and work-report evidence.
