# Docs Drift Assessment v0.1

Date: 2026-05-18
Status: proposed boundary
Authority: advisory/design

## Purpose

Define a docs-drift assessment boundary inspired by gstack-style documentation
release passes while preserving Punk's documentation governance: canonical docs
own truth, `DocImpact` must be declared, and automatic rewrites do not become
accepted truth without review.

This is a design/spec artifact only. It does not activate automatic docs
rewriting, public CLI behavior, module runtime, gate writing, proofpack writing,
or acceptance claims.

## Status and authority

A docs-drift assessment is an advisory review assessment.

It can identify stale docs, missing updates, duplicate truth, and public claim
mismatch. It cannot silently rewrite canonical truth or accept work.

## Required deterministic eval cases

### DOCS-DRIFT-001: DocImpact required for meaningful changes

A meaningful change touching laws, architecture, roadmap, crate status,
project memory, contract semantics, evals, public narrative, active/parked
scope, or CLI surface must declare `DocImpact` or an explicit no-impact
rationale.

### DOCS-DRIFT-002: canonical owner is updated

A docs-drift assessment must identify the canonical owner for each changed truth
surface. Updating only an entry page is insufficient when the canonical owner is
stale.

### DOCS-DRIFT-003: active/parked/future labels stay consistent

Docs that imply a parked, future, incubating, or side-effect-free model is
active must be flagged.

### DOCS-DRIFT-004: public narrative does not overclaim readiness

Public docs or publishing drafts that describe Punk as production-ready,
turnkey, autonomous, cloud-backed, or provider-owned must be flagged unless
canonical docs support that claim.

### DOCS-DRIFT-005: generated views are not canonical

A generated view or instruction output must not be treated as source truth.
Source docs/pages must remain the owner.

### DOCS-DRIFT-006: duplicate schema definitions are flagged

If a schema appears in multiple docs with conflicting fields or authority, the
assessment must identify the canonical owner and recommend consolidation.

### DOCS-DRIFT-007: work report includes knowledge impact when required

R2/R3 architecture/product/core-memory changes, and any change touching laws,
architecture, project memory, contract semantics, evals, roadmap, public
narrative, or active/parked scope, must include a Knowledge Impact section or a
clear route for the omission.

### DOCS-DRIFT-008: stale status date is visible

If a touched doc's `updated_at` or review metadata is stale relative to the
change, the assessment must flag it.

### DOCS-DRIFT-009: documentation-map rows are aligned

New canonical product docs must be reflected in `docs/product/DOCUMENTATION-MAP.md`
or explicitly justify why they are not canonical owners.

### DOCS-DRIFT-010: assessment is not an auto-accept rewrite

The docs-drift assessment may recommend patches. It must not claim acceptance
or proof by itself.

## Minimal fixture shape

```yaml
docs_drift_assessment:
  assessment_id: docs_drift_fixture_001
  status: advisory
  authority: non_authoritative
  changed_refs:
    - docs/product/RUNNER-AIDS.md
  canonical_owner_findings:
    - truth_surface: runner-aid-boundary
      owner_ref: docs/product/RUNNER-AIDS.md
      map_update_required: true
  findings: []
  gate_input_eligible: true
```

## Non-goals

This spec does not define automated docs editing, documentation generation,
public CLI behavior, runtime storage, gate writing, proofpack writing, or
acceptance claims.
