---
id: idea_2026_05_21_adaptive_module_control_plane
kind: idea
status: advisory
authority: non_authoritative
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_docs:
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/INSTRUCTION-SOURCES.md
  - docs/product/PROJECT-MEMORY.md
---

# Adaptive Module Control Plane v0.1

## Intent

Make future Punk modules adjustable without making them self-modifying.

The target pattern:

```text
stable module implementation
  + editable behavior artifacts
  + explicit evidence from real use
  + propose-only reflection
  + eval/gate/provenance before promotion
```

## Core rule

Editable module instructions, skills, playbooks, profiles, and control files
are behavior artifacts.

They are not automatically safe just because they are text or config.

Any behavior artifact change that affects module behavior should be treated as
an audited behavior change:

```text
evidence refs
  -> tuning proposal
  -> eval/check refs
  -> gate rationale
  -> versioned artifact update
```

## Proposed artifact layers

| Layer | Role | Authority |
|---|---|---|
| Module implementation | Stable, tested module logic. | Code authority only through normal repo review. |
| Module control manifest | Declares module identity, behavior artifact refs, immutable core boundaries, and capability requests. | Advisory until promoted by later docs/evals. |
| Behavior artifacts | Instructions, skills, playbooks, profiles, routing rules, templates, and recurring workflow preferences. | Editable, but not authoritative without promotion. |
| Evidence refs | Receipts, reports, events, eval outputs, user-provided examples, and explicit limitations. | Evidence only, not decisions. |
| Reflection run | Reads bounded evidence and proposes improvements. | Proposal generator only. |
| Tuning proposal | A diff or new artifact proposal with evidence, risks, and eval plan. | Advisory until gate/eval promotion. |
| Gate/provenance path | Records rationale and accepted artifact refs. | Only promoted gate path can make accepted behavior current. |

## Minimal module control manifest sketch

```yaml
module_control_manifest:
  module_id: pubpunk
  status: advisory
  module_version_ref: null
  baseline_ref: docs/product/MODULE-AUTHORING.md
  behavior_artifact_refs:
    - docs/modules/pubpunk-workspace-instructions.md
  immutable_core:
    - capability_envelope
    - host_validation
    - gate_interface
    - receipt_emission
    - proof_authority
  capability_requests: []
  reflection:
    trigger: none
    evidence_window_ref: null
    min_evidence_refs: null
  provenance:
    artifact_set_hash: null
    latest_gate_ref: null
  non_authority: true
```

This is not an active schema, parser, manifest registry, module loader, or
runtime contract.

## Tuning proposal sketch

```yaml
tuning_proposal:
  id: tuning_proposal_pubpunk_example
  status: proposed
  module_id: pubpunk
  target_artifact_refs:
    - docs/modules/pubpunk-workspace-instructions.md
  proposed_change_ref: null
  evidence_refs: []
  evidence_limitations:
    - insufficient_independent_evidence
  eval_refs: []
  capability_change_requested: false
  immutable_core_touched: false
  drift_budget_impact: unknown
  requested_gate_ref: null
  non_authority: true
```

This is not a patch writer and not an auto-apply mechanism.

## Adopt

- Stable implementation plus behavior artifact layer.
- Behavior artifact changes as audited changes.
- Propose-only reflection.
- Explicit behavior-provenance binding.
- Capability envelope remains separate from behavior artifacts.
- Drift budget and re-audit trigger.

## Defer

- Runtime manifest parsing.
- Background weekly reflection automation.
- OpenTelemetry or OpenFeature integration.
- Long-term agent memory engine.
- Deterministic checker implementation.
- Module Host runtime activation.

## Avoid

- Silent self-modification.
- Hidden reflection memory.
- Autonomous skill-library growth.
- Capability grants through skill or instruction text.
- User approval as a replacement for gate/eval/provenance.
- Treating generated views, prompts, or local memories as project truth.

## First safe next step

The next selected slice should define a narrow module control manifest and
tuning proposal contract before any runtime reflection is implemented.

PubPunk can be the fixture, but the result should stay module-general.

## Non-goals

No code, runtime, module loader, adapter invocation, browser automation,
credential read, external publishing, metrics collection, `.punk/` state write,
gate writer, proofpack writer, or acceptance claim is added by this idea.
