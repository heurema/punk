---
id: report_2026_05_21_adaptive_module_control_plane_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_capture_adaptive_module_control_plane_v0_1.md
---

# Adaptive Module Control Plane v0.1

## Summary

Captured adaptive modules as a docs/eval direction for Punk.

The key boundary is that editable instructions, skills, playbooks, profiles,
and control files are behavior artifacts. They may be easier to edit than code,
but they are not safety-light config. Changes need evidence refs, eval/check
refs, gate rationale, and provenance before promotion.

No runtime behavior was activated.

## Files changed

- `knowledge/research/2026-05-21-adaptive-module-control-plane.md`
- `knowledge/ideas/2026-05-21-adaptive-module-control-plane.md`
- `docs/product/MODULE-AUTHORING.md`
- `evals/specs/module-authoring-baseline.v0.1.md`
- `work/goals/goal_capture_adaptive_module_control_plane_v0_1.md`
- `work/reports/2026-05-21-adaptive-module-control-plane-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

Reason: the change affects module authoring, future module interface shape,
persistent behavior-control artifacts, reflection boundaries, project memory,
and future adapter/side-effect posture.

## Source scan

Reviewed prior Punk artifacts:

- `docs/product/MODULE-AUTHORING.md`
- `docs/product/MODULE-HOST-CONTRACT.md`
- `docs/product/INSTRUCTION-SOURCES.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/DELIBERATION-BUDGET.md`
- `knowledge/research/2026-04-25-karpathy-skills-review.md`
- `knowledge/research/2026-05-18-gstack-workflow-prior-art.md`
- `knowledge/research/2026-05-15-page-index-instruction-navigation.md`

Reviewed external prior art:

- Twelve-Factor config;
- Kubernetes controllers and custom resources;
- OPA decision logs;
- OpenTelemetry;
- OpenFeature;
- VS Code extension manifests;
- Claude Skills;
- LangGraph memory;
- Reflexion, Self-Refine, and Voyager papers.

## Deliberation

Budget class: `standard`.

Independent advisory passes:

- Codex source synthesis: selected an adaptive control-plane direction and
  kept it docs/eval-only.
- Claude Sonnet: recommended binary/skill split, explicit manifest, evidence
  store, bounded reflection, and promotion protocol.
- Gemini: recommended engine/artifact separation, declarative manifests,
  propose-only adaptation, and eval-gated upgrades.
- Claude Opus: challenged the config/code framing and required behavior
  artifacts to be treated as audited behavior changes.

Selected synthesis:

- Use stable implementation plus editable behavior artifacts.
- Reflection may propose, not apply.
- Editable behavior artifact changes need evidence, eval, gate, and provenance.
- Capability envelopes and immutable core rules cannot be modified by behavior
  artifacts.
- Full runtime reflection remains deferred.

## Doc impact

```yaml
doc_impact:
  classification: research-promotion
  canonical_docs:
    - docs/product/MODULE-AUTHORING.md
  research:
    - knowledge/research/2026-05-21-adaptive-module-control-plane.md
  ideas:
    - knowledge/ideas/2026-05-21-adaptive-module-control-plane.md
  evals:
    - evals/specs/module-authoring-baseline.v0.1.md
  work_artifacts:
    - work/goals/goal_capture_adaptive_module_control_plane_v0_1.md
    - work/reports/2026-05-21-adaptive-module-control-plane-v0-1.md
    - work/STATUS.md
  reason: "Promotes an R2 advisory adaptive-module direction into the module-authoring baseline without activating runtime, adapters, background reflection, publishing, metrics, credential access, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files knowledge/research/2026-05-21-adaptive-module-control-plane.md knowledge/ideas/2026-05-21-adaptive-module-control-plane.md docs/product/MODULE-AUTHORING.md evals/specs/module-authoring-baseline.v0.1.md work/goals/goal_capture_adaptive_module_control_plane_v0_1.md work/STATUS.md --report work/reports/2026-05-21-adaptive-module-control-plane-v0-1.md` - PASS.
- `git diff --check` - PASS.
- `cargo check --workspace` - not run; no Rust code changed.

## Knowledge impact

- Canonical product docs changed: `docs/product/MODULE-AUTHORING.md`.
- Project-memory claims affected: future modules should reserve an adaptive
  control-plane surface but must treat behavior artifact changes as audited
  behavior changes.
- Docs / ADRs / evals possibly stale: a future `MODULE-CONTROL-PLANE` doc may
  be needed if the topic outgrows Module Authoring.
- Active / parked / future scope affected: active runtime scope unchanged;
  module control manifests, tuning proposals, background reflection, runtime
  manifest parsing, and automatic reflection remain deferred.
- Public narrative impact: none.
- Derived views to rebuild later: any future module-authoring or instruction
  source views should include the new adaptive control-plane boundary.
- Follow-up goals or drift findings: define a module control manifest and
  tuning proposal contract as a separate bounded goal if selected.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, Module Host runtime
activation, PubPunk runtime activation, `.punk/` runtime state, adapter
invocation, browser automation, credential access, external publishing,
metrics collection, gate writer, proofpack writer, or acceptance claim was
added.
