# Module Control Validation v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the Module Control Plane validation
runbook and marker-suite wiring.

This spec does not activate runtime manifest parsing, overlay resolution,
background reflection, behavior-artifact writing, user-local config writing,
article reading, metrics collection, research execution, adapter invocation,
event writing, receipt writing, publishing, gate writing, proofpack writing, or
acceptance claims.

## Required deterministic eval cases

Current focused commands:

- `scripts/check.sh module-control-plane`
- `scripts/check.sh module-behavior-boundaries`
- `scripts/check.sh module-tuning-handoff-template`
- `scripts/check.sh pubpunk-control-manifest`
- `scripts/check.sh pubpunk-hook-tuning-fixture`
- `scripts/check.sh pubpunk-live-tuning-pack`
- `scripts/check.sh module-control-validation`
- `scripts/check.sh module-control-suite`

### MODULE-CONTROL-VALIDATION-001: command map is explicit

The validation runbook must list every current Module Control Plane and PubPunk
control-plane marker command with the surface it protects.

### MODULE-CONTROL-VALIDATION-002: suite command is explicit

The runbook must identify `scripts/check.sh module-control-suite` as the
current aggregate marker suite.

### MODULE-CONTROL-VALIDATION-003: validation command is explicit

The runbook and spec must identify `scripts/check.sh module-control-validation`
as the focused check for the validation runbook and command wiring.

### MODULE-CONTROL-VALIDATION-004: repo-level checks remain separate

The runbook must list Research Gate and Work Ledger checks as repo-level checks
that are still required after Module Control Plane slices.

### MODULE-CONTROL-VALIDATION-005: docs governance is routed

The runbook must route docs, eval, goal, report, and status changes through
docs governance with a work-report ref.

### MODULE-CONTROL-VALIDATION-006: marker check meaning is limited

The runbook must say that passing marker checks only means expected markers are
present and obvious activation markers are absent.

### MODULE-CONTROL-VALIDATION-007: marker checks are not runtime proof

The runbook must state that passing marker checks does not activate manifest
parsing, overlay resolution, config writing, reflection, metrics, research,
adapter invocation, publishing, event writing, gates, proofpacks, or acceptance
claims.

### MODULE-CONTROL-VALIDATION-008: smallest focused check is preferred

The runbook must tell operators to use the smallest check that covers the
edited surface while developing.

### MODULE-CONTROL-VALIDATION-009: suite runs before closure

The runbook must tell operators to run the aggregate suite before closing a
Module Control Plane or PubPunk control-plane slice.

### MODULE-CONTROL-VALIDATION-010: command wiring changes use validation check

The runbook must route command-name, validator-file-name, suite-member, and
validation-runbook changes through `scripts/check.sh module-control-validation`.

### MODULE-CONTROL-VALIDATION-011: failure handling preserves boundaries

The runbook must instruct operators not to remove no-runtime, no-auto-apply,
no-writer, no-capability-change, no-side-effect-change, no-secret,
no-private-data, or no-executable-code markers merely to make checks pass.

### MODULE-CONTROL-VALIDATION-012: runtime desire stops validation path

The runbook must say that desired runtime behavior requires a separate selected
runtime boundary goal.

### MODULE-CONTROL-VALIDATION-013: suite membership is explicit

The runbook must list the current consolidated suite script explicitly.

### MODULE-CONTROL-VALIDATION-014: validation is non-authoritative

The runbook must state that the suite is read-only and not a gate, proofpack,
runtime acceptance signal, or substitute for product review.

### MODULE-CONTROL-VALIDATION-015: Python is limited to marker checks

The runbook must state that Python is allowed only for static repo-governance
and docs/spec marker checks, while runtime, parser, resolver, writer, module
behavior, and implementation-state checks belong in Rust or a future Rust
`xtask` path.

## Validator command

```sh
scripts/check.sh module-control-validation
```

For the current aggregate control-plane marker suite:

```sh
scripts/check.sh module-control-suite
```

The commands validate the current Module Control Plane validation runbook and
marker-suite wiring. They do not parse manifests, resolve behavior artifacts,
run reflection, write config, promote artifacts, activate PubPunk runtime, or
claim acceptance.
