# Eval

## Purpose

Eval answers one question:

Did the system get better or worse after a change?

## Current source of truth

For active-core policy, use:

- `docs/product/EVAL-PLANE.md`
- `docs/adr/ADR-0008-eval-plane.md`

Current decision:

- the core eval harness is Punk-native
- the active path is local-first, repo-stored, deterministic-first, and fixture-based
- Pydantic-compatible vocabulary is useful later, but not a core dependency
- external frameworks and benchmarks are calibration or bridge layers later, not the release gate
- evals report evidence and can block promotion, but only `gate` writes final decisions

## Eval layers

- smoke — fast must-pass checks
- core — product-value regression tasks
- stretch — longer/noisier long-horizon tasks
- external calibration — optional monthly public benchmark slices

## Case migration note

Phase 2 smoke cases should follow `evals/_schema/eval-case.v1.yaml`.

Older non-smoke sketches may remain in a lightweight format until the runner and the rest of the suite are migrated.

## Initial suites

- flow
- contract quality
- project memory
- module conformance
- project coherence

## Hard gates

A change cannot be promoted if it introduces:

- repository integrity regression
- illegal flow transition
- missing receipt
- missing decision/proof
- critical scope violation
- module decision ownership
- speculative knowledge leakage in implementation mode

## MVP metrics

Track separately:

- outcome integrity
- scope adherence
- retrieval/navigation
- consistency
- coordination/handoff
- efficiency/cost

No single composite score can override a hard gate.

## Initial policy

Every new capability must add or update at least one eval case, or include an explicit no-eval rationale in the decision record.

## Public benchmark policy

Public benchmarks are calibration, not the release gate.

Initial policy:

- internal suite gates promotion
- public slices run monthly or manually
- SWE-bench Verified-like slices can calibrate issue-resolution ability later
- terminal benchmarks are relevant only if terminal autonomy becomes a core product surface

## Future module/plugin conformance evals

When the Module Host phase begins, module conformance must cover the host boundary before any module or plugin runtime becomes active.

Required future eval candidates:

- module/plugin cannot write a final decision
- module/plugin cannot mutate the event log directly
- module/plugin cannot bypass contract scope
- invalid receipt or assessment schema fails closed
- missing receipt fails the run
- undeclared filesystem, network, environment, secret, or publishing access is denied
- declared capabilities are recorded in the run receipt
- resource limits are enforced and receipted
- oversized output is denied or bounded with an explicit receipt
- plugin-local state cannot become canonical project memory

These are candidate evals until Phase 6 promotes concrete Module Host work.
