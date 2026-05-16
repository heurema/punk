---
id: goal_correct_publishing_module_boundary_guardrail_v0_1
title: "Correct Publishing Module Boundary Guardrail v0.1"
status: done
owner: "vitaly"
module: "architecture"
priority: P1
authority: canonical
created_at: 2026-05-16
updated_at: 2026-05-16
selected_at: 2026-05-16
started_at: 2026-05-16
completed_at: 2026-05-16
blocked_by: []
scope:
  include:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/modules/pubpunk.md"
    - "scripts/check_docs_governance.py"
    - "work/STATUS.md"
    - "work/goals/goal_add_local_publishing_workspace_locator_v0_1.md"
    - "work/goals/goal_correct_publishing_module_boundary_guardrail_v0_1.md"
    - "work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md"
    - "work/reports/2026-05-16-publishing-module-boundary-guardrail-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/publications/**"
acceptance:
  - "Identifies why publishing inventory was incorrectly routed toward active core."
  - "Withdraws the previous core publishing inventory follow-up suggestion."
  - "States that publishing inventory, drafting, planning, receipt creation, and publish behavior belong behind PubPunk/module-host boundaries."
  - "Keeps the existing `punk publishing locate` command as a transitional locator only."
  - "Adds docs-governance coverage for unguarded non-locate publishing subcommand claims in touched docs/work artifacts."
  - "Adds no Rust code, runtime behavior, CLI behavior, publishing behavior, adapter, bot, API call, or external side effect."
research_gate:
  classification: R2
  required: true
  rationale: "This correction changes architecture guardrails for module/core boundaries and active CLI claims, based on existing canonical docs plus explicit maintainer correction."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/modules/pubpunk.md"
    - "scripts/check_docs_governance.py"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The correction tightens the PubPunk/module-host boundary and prevents the completed locator slice from implying more active-core publishing commands."
---

# Correct Publishing Module Boundary Guardrail v0.1

## Context

An implementation attempt started adding a publishing inventory subcommand to
`punk-project`, `punk-cli`, and `punk-eval`.

That was architecturally wrong. Publishing inventory is module behavior, not
active-core behavior.

## Root Cause

The 2026-05-15 local publishing locator artifacts created a transitional core
CLI exception for `punk publishing locate` and then described the next
publishing step as a local publishing plan/draft inventory reader.

That handoff omitted the required PubPunk/module-host boundary and made the
next step look like another active-core CLI slice.

## Correction

The locator remains accepted only as a local workspace resolver.

The previous core publishing inventory follow-up is withdrawn.

Future publishing inventory, drafting, planning, receipt creation, and publish
behavior must be selected through PubPunk/module-host boundary work before any
new active behavior is added.

## Prevention

Docs-governance now fails touched docs/work artifacts that introduce unguarded
non-locate publishing subcommand claims or describe publishing inventory,
drafting, planning, receipt, or publish work as the next core code slice
without a PubPunk/module boundary.
