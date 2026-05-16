---
id: goal_define_pubpunk_module_boundary_v0_1
title: "Define PubPunk Module Boundary v0.1"
status: done
owner: "vitaly"
module: "pubpunk"
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
    - "docs/product/MODULES.md"
    - "docs/modules/pubpunk.md"
    - "evals/specs/pubpunk-module-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_pubpunk_module_boundary_v0_1.md"
    - "work/reports/2026-05-16-pubpunk-module-boundary-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/publications/**"
acceptance:
  - "Defines PubPunk as module-owned behavior, not active core behavior."
  - "Defines future PubPunk input bundle, output surfaces, and deny-by-default capability boundary."
  - "States publishing inventory, drafting, planning, receipt creation, and publish behavior remain behind PubPunk/module-host promotion."
  - "Adds eval/spec boundary cases for module ownership, capabilities, side-effect receipts, workspace truth, adapters, and gate authority."
  - "Adds no Rust code, runtime behavior, CLI behavior, publishing behavior, adapter, bot, API call, receipt writer, or external side effect."
research_gate:
  classification: R2
  required: true
  rationale: "This boundary affects module architecture, future side-effect policy, publishing behavior, adapter boundaries, and gate/proof handoff, but records docs/spec-only constraints based on existing canonical docs and the explicit maintainer correction."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/MODULES.md"
    - "docs/modules/pubpunk.md"
    - "evals/specs/pubpunk-module-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "This change records a module boundary and eval/spec constraints before any PubPunk implementation."
---

# Define PubPunk Module Boundary v0.1

## Context

The maintainer wants zero-manual Punk eventually, including publishing, but
publishing cannot be implemented as active-core behavior.

The prior publishing locator is a transitional local resolver only. The next
step is a PubPunk/module boundary, not another `punk publishing` command.

## Selected Slice

Define PubPunk module boundary v0.1 with:

- future input bundle shape;
- future output surfaces;
- denied-by-default capabilities;
- side-effect receipt requirements;
- adapter truth boundary;
- eval/spec cases for module conformance.

## Boundary

This slice adds no implementation.

It does not activate PubPunk runtime, Module Host runtime, publishing
inventory, draft generation, receipt writing, external publishing, adapters,
bots, browser/API calls, credential reads, CLI commands, gate writers,
proofpack writers, or acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with docs/spec/work-ledger evidence only.
