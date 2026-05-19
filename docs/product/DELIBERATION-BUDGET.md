---
id: docs_product_deliberation_budget
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
canonical_for:
  - deliberation-budget-boundary
  - multi-perspective-synthesis-boundary
  - cross-model-advisory-pass-boundary
  - synthesis-vs-decision-boundary
  - token-spend-policy-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/REVIEW-ASSESSMENT.md
  - docs/product/PLOT-INTAKE.md
  - docs/product/MODULES.md
related_research:
  - knowledge/research/2026-05-18-gstack-workflow-prior-art.md
  - knowledge/research/2026-05-19-plot-intake-boundary-prior-art.md
related_evals:
  - evals/specs/deliberation-budget-boundary.v0.1.md
supersedes: []
superseded_by: null
---

# Deliberation Budget

## Purpose

Name the boundary for intentionally spending extra human/model/tool attention on
important choices before implementation.

The goal is to reduce single-executor blind spots without turning model output,
provider output, or synthesis output into authority.

## Principle

Use more tokens when the cost of a wrong decision is higher than the cost of
additional advisory passes.

```text
goal or question
  -> bounded source pack
  -> independent advisory passes
  -> tension map
  -> synthesis
  -> contract / next-work candidate
  -> gate later, when applicable
```

All outputs in this flow are advisory until promoted through the normal Punk
artifact path. Only `gate` writes final decisions.

## Budget classes

Use budget classes to make token/time spending explicit:

| Class | Use when | Expected shape |
|---|---|---|
| `none` | The task is trivial, reversible, or already mechanically constrained. | One executor may proceed with normal evidence. |
| `light` | The task has some ambiguity, but low blast radius. | One extra lens, critique, or second opinion. |
| `standard` | The task affects module boundaries, contract shape, product truth, public narrative, or recurring workflow rules. | At least two independent advisory passes plus synthesis. |
| `deep` | The task is architecture-shaping, trust-critical, security/privacy-sensitive, public-claim-heavy, or expensive to reverse. | Multiple independent passes, adversarial critique, source verification, and explicit human questions when needed. |

Module-authoring rules, first-module strategy, and PubPunk boundary work should
default to `standard` unless a smaller budget is explicitly justified.

At Dogfooding Level 0, the budget class is assigned manually by the goal author,
maintainer, or Plot Intake note. Missing or undersized budget is a review
finding, not an automatic gate blocker.

Common trigger reasons:

- `module_boundary`
- `first_module_strategy`
- `contract_shape`
- `project_truth`
- `public_narrative`
- `security_or_privacy`
- `irreversible_or_expensive_to_reverse`
- `recurring_workflow_rule`

This list is an initial vocabulary, not a runtime enum.

## Artifact alignment

Use a `deliberation_packet` shape when the deliberation itself needs to be
inspectable:

```yaml
deliberation_packet:
  id: deliberation_packet_example
  status: advisory
  authority: non_authoritative
  question_ref: work/goals/example.md
  budget_class: standard
  trigger_reasons:
    - module_boundary
  source_refs:
    - docs/product/PUNK-LAWS.md
  independent_passes:
    - pass_id: pass_codex
      assessor_kind: model
      assessor_ref: codex
      input_refs:
        - docs/product/MODULES.md
      output_ref: null
      summary: "Proposed a minimal module-authoring baseline."
      limitations:
        - single_model_view
  tension_map:
    - topic: artifact_naming
      positions:
        - "Keep artifact unnamed until code."
        - "Name the artifact before module skeleton work."
  synthesis:
    selected_points: []
    rejected_points: []
    unresolved_questions: []
    recommended_next_work: null
  downstream_refs: []
  non_authority: true
```

This is an artifact boundary, not a runtime schema or writer.

At Dogfooding Level 0, a deliberation packet may live inline in a work report or
as a separate repo-tracked report artifact. Do not create a new
`work/deliberations/` tree, runtime store, or packet writer until a later
bounded goal selects that storage shape.

## Independent pass guidance

For deliberation, record why advisory passes differ in at least one meaningful
way:

- assessor or reviewer;
- model family or provider;
- role or lens;
- source pack;
- prompt or checklist;
- tool output being assessed.

Two prompts to the same model may count as separate lenses only when the packet
records that limitation. They do not provide provider diversity.

Unavailable providers should be recorded as limitations, not worked around by
pretending equivalent independence.

## Preservation requirements

Deliberation should preserve:

- the question being decided;
- source refs available to each pass;
- assessor refs and known limitations;
- material disagreements;
- minority insights that should not be averaged away;
- synthesis rationale;
- unresolved questions;
- recommended next work;
- non-authority status.

Token spend is not proof of quality. It is only evidence that additional review
attention was spent.

The `standard` default for module work is guidance until packet storage,
validators, and review flow are explicitly promoted. It must not freeze module
authoring while the infrastructure remains documentation-only.

## Relationship to Plot Intake

Plot Intake may request a deliberation budget when a raw request is ambiguous,
high-impact, or likely to become a repeating pattern for modules.

The resulting synthesis may improve contract-draft readiness. It must not
approve a contract, select a module as authority, or write a gate decision.

## Relationship to Review Assessment

Independent passes may be captured as review assessments when they inspect
specific artifacts, findings, or claims.

A synthesis may combine review assessments, but it remains advisory. It should
preserve disagreement rather than hiding it behind false consensus.

## Relationship to Runner Aids

Prompts, skills, playbooks, and model-specific instructions used for
deliberation are runner aids.

They must cite source refs, stay non-authoritative, and avoid hidden project
truth.

## Relationship to modules

Modules may use deliberation packets to improve module-specific contract
drafting, boundary review, or assessment quality.

Modules must not use deliberation packets to decide acceptance, bypass
contracts, bypass `gate`, activate adapters, or own project truth.

For early module development, this boundary protects the path to PubPunk:
common module-authoring choices should pass through at least `standard`
deliberation before implementation, so future modules do not drift into
incompatible local patterns.

## Publishing example

Future PubPunk workflows may use separate author, editor, critic, analyst, and
synthesis passes over publication drafts.

That may improve draft quality, evidence planning, and metrics questions.

It must not publish, write receipts, call adapters, claim metrics, or accept the
work without separate scoped permission and evidence.

## Forbidden behavior

Deliberation Budget must not:

- activate provider orchestration;
- require a specific model or vendor;
- create a public CLI command;
- create hidden multi-agent memory;
- invoke modules or adapters;
- publish;
- write receipts;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- treat synthesis as proof;
- turn token spend into authority;
- bypass `plot / cut / gate`.

## Current documentation boundary

Documented now:

- token/time budget classes;
- multi-perspective advisory pass boundary;
- tension-map and synthesis requirements;
- relationship to Plot Intake, Review Assessment, Runner Aids, modules, and
  future PubPunk.

Not active now:

- runtime model orchestration;
- provider adapters;
- automatic agent teams;
- prompt generation;
- deliberation packet writer;
- token accounting;
- CLI behavior;
- module activation;
- publishing behavior;
- gate/proof behavior.
