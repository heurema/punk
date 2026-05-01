---
id: goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1
title: "Polish greenfield init docs and flow inspect preview v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "crates/punk-cli/src/main.rs"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - ".gitignore"
    - "work/STATUS.md"
    - "work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "crates/punk-project/**"
    - "crates/punk-eval/**"
    - "schemas/**"
acceptance:
  - "`punk init <project-id>` docs/help clearly state that init operates in the current directory and does not create a subdirectory named `<project-id>`."
  - "Normal CLI wording keeps host paths privacy-safe."
  - "`punk flow inspect` preview output does not expose repo-internal goal refs as if they were user project state."
  - "Any `.punk/project/` docs or ignore mismatch is either reconciled or explicitly recorded as not real."
  - "No brownfield, grayfield, runtime storage, contracts, gate/proof runtime, Writer, agents/adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is added."
knowledge_refs:
  - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
  - "docs/product/START-HERE.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local docs/UX cleanup following greenfield init hardening; existing status docs and the hardening report provide the needed context."
  research_refs:
    - "work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The slice clarifies active CLI help/docs and removes a misleading preview-only repo goal ref from CLI output."
---

## Context

Greenfield init now has atomic preflight and privacy-safe `target_root: .` human output.

The remaining audit findings are smaller docs/UX consistency issues.

## Intent

Clarify current-directory init behavior and remove misleading preview/internal refs from `punk flow inspect` output.

## Non-scope

Do not implement brownfield init, grayfield reconciliation, repo scan, AI summaries, generated docs, contract generation, gate/proof runtime, Writer, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, runtime `.punk` storage, or new CLI surfaces.

## Outcome

CLI help and product docs now state that `punk init <project-id>` initializes the current directory in place and does not create a subdirectory named `<project-id>`.

`punk flow inspect` no longer emits `preview_goal_ref` or a repo-internal `work/goals/...` reference in its preview output.

`.gitignore` was inspected and left unchanged because `.punk/project.toml` is trackable marker/setup metadata, runtime/derived `.punk` dirs are ignored explicitly, and no active `.punk/project/` path mismatch was found.
