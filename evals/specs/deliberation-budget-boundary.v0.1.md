# Deliberation Budget boundary v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary cases before Punk promotes multi-perspective,
cross-model, cross-provider, or multi-agent deliberation behavior.

This is a design/spec artifact only. It does not activate provider
orchestration, public CLI behavior, Module Host runtime, model invocation,
adapter invocation, token accounting, runtime storage, gate writing, proofpack
writing, publishing, or acceptance claims.

## Status and authority

Deliberation outputs are advisory evidence.

They are not gate decisions, proofpacks, receipts, or acceptance authority.

## Required deterministic eval cases

### DELIBERATION-001: synthesis cannot decide

A deliberation synthesis must not claim final acceptance, final rejection,
proof completion, gate outcome, or module authority.

### DELIBERATION-002: budget class is explicit

A deliberation packet must declare `budget_class` as `none`, `light`,
`standard`, or `deep`, with trigger reasons when the class is not `none`.

### DELIBERATION-003: independent passes are visible

For `standard` and `deep` budgets, independent advisory passes must be listed
with assessor refs, input refs, summaries or output refs, and limitations.

### DELIBERATION-003A: same-provider passes disclose limits

Two passes from the same model family or provider may count as separate lenses
only when their different role, prompt, source pack, or checklist is visible and
their lack of provider diversity is recorded as a limitation.

### DELIBERATION-004: source differences are visible

If passes used different source refs, stale refs, external tools, model
families, or provider contexts, the packet must record that difference.

### DELIBERATION-005: tension map preserves disagreement

Material disagreements must be preserved in a tension map. A synthesis that
removes disagreement without rationale is invalid for gate-input eligibility.

### DELIBERATION-006: synthesis cites selected and rejected points

A synthesis must record selected points, rejected points, unresolved questions,
and recommended next work when those exist.

### DELIBERATION-007: token spend is not proof

Token count, model count, provider count, or number of agents must not be
treated as proof of correctness, safety, acceptance, or implementation quality.

### DELIBERATION-008: provider runtime is not implied

A deliberation boundary may mention model, provider, or agent diversity, but it
must not require active provider orchestration, API calls, adapter invocation,
credential access, or hidden model memory.

### DELIBERATION-009: side effects require separate receipts

If any pass changed files, opened a browser, called an API, published,
commented, created a PR, deployed, or touched credentials, the packet must link
side-effect receipts or mark the output ineligible as gate input.

### DELIBERATION-010: module-authoring defaults to standard

Module-authoring rules, first-module strategy, module boundary changes, and
PubPunk boundary work should default to `standard` deliberation unless the
packet records a smaller-budget rationale.

Missing `standard` deliberation is a review finding while this boundary is
documentation-only. It is not an automatic gate blocker before packet storage,
validators, and review flow are separately promoted.

### DELIBERATION-010A: packet storage is not implied

A packet may live inline in a work report or as a separate repo-tracked report
artifact at Dogfooding Level 0. The boundary must not imply a new
`work/deliberations/` tree, runtime storage, packet writer, or generated index.

### DELIBERATION-011: publishing synthesis cannot publish

A publishing-oriented deliberation may improve drafts, critique, metrics
questions, or evidence plans, but it must not publish externally, create
publication receipts, claim metrics, or accept publication work.

### DELIBERATION-012: hidden memory is non-canonical

Prompt text, provider-local memory, chat history, model scratchpads, and
executor-local notes must not become canonical project truth unless captured
through explicit repo-tracked refs with authority, scope, source, date, and
review path.

## Minimal fixture shape

```yaml
deliberation_packet:
  id: deliberation_packet_fixture_001
  status: advisory
  authority: non_authoritative
  question_ref: work/goals/goal_fixture.md
  budget_class: standard
  assigned_by: goal_author
  trigger_reasons:
    - module_boundary
  source_refs:
    - docs/product/MODULES.md
  independent_passes:
    - pass_id: pass_001
      assessor_kind: model
      assessor_ref: codex
      input_refs:
        - docs/product/MODULES.md
      output_ref: null
      summary: "Proposed a minimal module-authoring baseline."
      limitations:
        - single_model_view
        - same_provider_lens_only
    - pass_id: pass_002
      assessor_kind: model
      assessor_ref: gemini
      input_refs:
        - docs/product/MODULES.md
      output_ref: null
      summary: "Emphasized tension-map preservation."
      limitations:
        - external_model_advisory_only
  tension_map:
    - topic: artifact_naming
      positions:
        - "Name the artifact now."
        - "Keep the artifact unnamed until code."
  synthesis:
    selected_points:
      - "Document the boundary first."
    rejected_points:
      - "Do not activate provider orchestration."
    unresolved_questions:
      - "Final artifact name remains open."
    recommended_next_work: work/goals/goal_fixture_next.md
  downstream_refs: []
  non_authority: true
```

## Non-goals

This spec does not define provider routing, model selection, prompt templates,
agent orchestration, token accounting implementation, storage writers, CLI
commands, module activation, adapter behavior, publishing behavior, gate
writing, proofpack writing, or acceptance claims.
