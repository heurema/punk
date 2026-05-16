---
id: goal_add_local_publishing_workspace_locator_v0_1
title: "Add Local Publishing Workspace Locator v0.1"
status: done
owner: "vitaly"
module: "public-narrative"
priority: P2
authority: canonical
created_at: 2026-05-15
updated_at: 2026-05-16
selected_at: 2026-05-15
started_at: 2026-05-15
completed_at: 2026-05-15
blocked_by: []
scope:
  include:
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-cli/src/main.rs"
    - "crates/punk-eval/src/lib.rs"
    - "scripts/check_docs_governance.py"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_local_publishing_workspace_locator_v0_1.md"
    - "work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md"
  exclude:
    - ".github/**"
    - ".punk/**"
    - "crates/**/adapters/**"
    - "docs/adr/**"
    - "publishing/publications/**"
acceptance:
  - "`punk publishing locate` resolves a local publishing workspace from `.punk/publishing.toml` and local-only `.punk/publishing.local.toml`."
  - "The resolver validates logical `punk-publishing://` workspace refs and blocks mismatches, missing local pointers, unavailable roots, and committed host workspace roots."
  - "The command supports human and JSON output and `--project-root <path>`."
  - "Smoke and CLI coverage prove local-only resolver behavior and no write/publish/API/browser/credential behavior."
  - "Docs-governance active CLI truth is updated so the new implemented command is not treated as overclaim."
  - "Docs/status distinguish local workspace location from PubPunk automation and publishing execution."
  - "No external side effects, publishing automation, browser/API calls, credentials, bots, adapters, automatic receipts, issue/PR creation, gate decisions, proofpack writing, or acceptance claims are added."
knowledge_refs:
  - "README.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "work/STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This change adds a public CLI surface and affects the publishing boundary, but it is bounded to local-only read/locate behavior and is covered by existing canonical product docs plus the maintainer-provided Goalrail publishing binding context."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "work/STATUS.md"
    - "work/reports/**"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "scripts/check_docs_governance.py"
  rationale: "The patch adds a bounded public CLI command for publishing workspace location, so active-surface docs and work-ledger evidence were updated."
---

## Context

The maintainer needs Punk to move toward zero-manual publishing workflows, but
external publishing is still blocked until local state, receipts, policy, and
gate/proof integration exist.

The immediate practical gap is locating a project's external local publishing
workspace without hard-coding host paths into repo-tracked truth.

## Selected Slice

Implemented the smallest active publishing prerequisite:

- a local-only publishing workspace resolver model in `punk-project`;
- a bounded `punk publishing locate` CLI command;
- a docs-governance active-CLI allowlist update for the newly implemented
  command;
- smoke and CLI coverage proving no write, publish, API, browser, credential,
  bot, adapter, or receipt behavior.

## Boundary

`punk publishing locate` is a resolver, not PubPunk automation.

It reads repo binding plus local-only pointer config, reports located/blocked
status, and exits fail-closed when the binding is missing, unsafe, mismatched,
or unavailable.

It does not publish, draft, open browsers, call APIs, read credentials, activate
adapters, create receipts, create issues or PRs, write gate decisions, write
proofpacks, or make external surfaces project truth.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with code, smoke coverage, CLI tests, docs/status, and work-report
evidence.

## 2026-05-16 Architectural correction

The locator remains accepted as a transitional local resolver, but it must not
be treated as precedent for putting PubPunk behavior into active core.

The previous follow-up suggestion for a core publishing inventory reader is
withdrawn. Publishing inventory, drafting, planning, receipt creation, and
publish behavior belong behind a future PubPunk/module-host boundary, not in
`punk-project`, `punk-cli`, or additional active-core publishing subcommands.
