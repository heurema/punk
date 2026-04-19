# Eval

## Purpose

Eval answers one question:

Did the system get better or worse after a change?

## Decision

Start with a local, repo-stored, deterministic-first regression suite.

Recommended external runner layer for the first Python harness: Pydantic Evals.

Use external benchmarks only as calibration, not as the core release gate.

## Eval layers

- smoke — fast must-pass checks
- core — product-value regression tasks
- stretch — longer/noisier long-horizon tasks
- external calibration — optional monthly public benchmark slices

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


## Runner strategy

Recommended first external runner layer:

- Pydantic Evals as a Python harness around the `punk` CLI
- local datasets stored in `evals/`
- deterministic checks first
- rubric/LLM judging only for narrow secondary checks

The Rust core should define the stable CLI, artifacts, and result formats.

The Python harness can provide fast iteration on datasets, repeated runs, reports, and comparison.

## Public benchmark policy

Public benchmarks are calibration, not the release gate.

Initial policy:

- internal suite gates promotion
- public slices run monthly or manually
- SWE-bench Verified-like slices can calibrate issue-resolution ability later
- terminal benchmarks are relevant only if terminal autonomy becomes a core product surface
