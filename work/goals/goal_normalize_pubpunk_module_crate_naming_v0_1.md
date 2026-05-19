---
id: goal_normalize_pubpunk_module_crate_naming_v0_1
title: "Normalize PubPunk Module Crate Naming v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P1
authority: canonical
created_at: 2026-05-19
updated_at: 2026-05-19
selected_at: 2026-05-19
started_at: 2026-05-19
completed_at: 2026-05-19
blocked_by: []
scope:
  include:
    - "Cargo.toml"
    - "Cargo.lock"
    - "README.md"
    - "crates/punk-mod-pubpunk/Cargo.toml"
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/cases/project/eval_project_no_parked_capability_exposed.yaml"
    - "evals/specs/module-naming-baseline.v0.1.md"
    - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_normalize_pubpunk_module_crate_naming_v0_1.md"
    - "work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "external publishing behavior"
    - "module runtime behavior"
acceptance:
  - "Defines module naming baseline rules around canonical root, product name, module id, crate name, directory path, import name, and non-canonical aliases."
  - "Normalizes PubPunk's active crate/package/path from punk-mod-pub to punk-mod-pubpunk."
  - "Updates current docs, eval refs, and work status to the normalized crate name."
  - "Keeps PubPunk product name and module id as PubPunk/pubpunk."
  - "Keeps pub as a proposed-only short alias, not canonical."
  - "Does not change PubPunk behavior, Module Host behavior, public CLI behavior, publishing, metrics, receipts, gate/proof behavior, or acceptance claims."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/modules/pubpunk.md"
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/module-naming-baseline.v0.1.md"
  - "work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes a domain-module crate/package/path name and codifies future module naming policy. It is satisfied by existing module authoring docs, PubPunk docs, crate-status docs, and the three-provider advisory naming pass recorded in the report."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/**"
    - "docs/modules/**"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Renames an incubating PubPunk crate/package/path and codifies module naming rules without behavior changes."
---

# Normalize PubPunk Module Crate Naming v0.1

## Context

PubPunk had drift between product name `PubPunk`, module id `pubpunk`, crate
name `punk-mod-pub`, and informal alias `pub`.

## Selected slice

Normalize naming only:

- canonical root and derivation rules;
- PubPunk crate/package/path/import rename;
- current docs/eval/status refs;
- validation report.

## Boundary

This slice changes no runtime behavior, public CLI behavior, Module Host
behavior, module activation, provider orchestration, adapter invocation,
publishing behavior, metrics collection, receipt writing, gate writing,
proofpack writing, or acceptance claims.

## Outcome

Done when current refs use `punk-mod-pubpunk`, the naming baseline is recorded,
checks pass, and `selected_next` remains unchanged.
