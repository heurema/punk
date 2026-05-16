---
id: goal_add_pubpunk_inventory_assessment_model_v0_1
title: "Add PubPunk Inventory Assessment Model v0.1"
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
    - "Cargo.toml"
    - "Cargo.lock"
    - "crates/punk-mod-pub/Cargo.toml"
    - "crates/punk-mod-pub/src/lib.rs"
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "README.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/MODULES.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_inventory_assessment_model_v0_1.md"
    - "work/reports/2026-05-16-pubpunk-inventory-assessment-model-v0-1.md"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/publications/**"
    - "provider adapters"
    - "bots"
    - "external API behavior"
    - "publication behavior"
    - "receipt writer"
    - "gate decision writer"
    - "proofpack writer"
acceptance:
  - "Adds a module-owned PubPunk crate instead of putting publishing inventory logic in active core."
  - "The model assesses only caller-provided publishing metadata and source refs."
  - "The model returns advisory inventory assessment evidence with candidate counts, receipt gaps, findings, refs, and boundary flags."
  - "The model blocks unsupported capability grants, raw/private payload policy, raw body inputs, unsafe source refs, and missing required refs."
  - "Smoke coverage proves the model is side-effect-free and not a CLI, publisher, receipt writer, adapter invoker, gate writer, proof writer, or acceptance authority."
  - "Docs/status distinguish incubating PubPunk library behavior from parked PubPunk runtime, Module Host runtime, adapters, and publishing automation."
  - "Adds no external side effects, GitHub/API/browser/credential behavior, bot, adapter, automatic issue/PR creation, publication receipt writer, roadmap voting, DAO, token, or funding behavior."
research_gate:
  classification: R2
  required: true
  rationale: "This implementation affects module architecture and future publishing automation boundaries, but it is a bounded local code slice based on existing canonical docs and the PubPunk boundary spec."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/MODULES.md"
    - "docs/modules/pubpunk.md"
    - "evals/specs/pubpunk-module-boundary.v0.1.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/MODULES.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The patch adds an incubating module crate and smoke coverage, so current crate/module status and work-ledger evidence must reflect it."
---

# Add PubPunk Inventory Assessment Model v0.1

## Context

The maintainer wants Punk to move toward zero-manual publishing, but the
previous correction explicitly forbids putting publishing inventory, drafting,
planning, receipt creation, or publishing behavior into active core.

The smallest next slice is a PubPunk-owned model that can describe local
publishing inventory evidence without doing IO or external actions.

## Selected Slice

Add `punk-mod-pub` with a side-effect-free inventory assessment model.

The model accepts an explicit bounded input bundle:

- module/version refs;
- contract, run, project, and publishing workspace refs;
- caller-provided inventory item metadata;
- capability grants;
- privacy policy;
- expected receipt fields.

It returns advisory assessment evidence:

- candidate post/draft count;
- publication receipt gap count;
- findings;
- boundary flags;
- refs.

## Boundary

This slice does not activate PubPunk runtime, Module Host runtime, public CLI,
publishing workspace scanning, file reads, draft generation, publication
planning, receipt writing, external publishing, metrics collection, adapters,
bots, credential reads, issue/PR automation, gate decisions, proofpacks, or
acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with module-owned Rust model, unit tests, smoke coverage, docs/status, and
work-report evidence.
