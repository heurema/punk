# Smoke eval report schema v0.1

Date: 2026-04-22
Status: proposed schema
Authority: advisory/design

## Purpose

Define a bounded `smoke-eval-report.v0.1` artifact shape for future machine-readable smoke eval reports.

This spec is a design artifact only.

It prepares a later implementation step without implying:

- stable public machine-readable output;
- `.punk/evals` runtime storage;
- baseline or waiver semantics;
- report persistence;
- gate/proof behavior.

## Status and authority

This is a proposed v0.1 artifact shape.

It is not yet a stable public contract.

It does not become product truth by itself.

Future implementation requires a separate bounded goal.

## Relationship to the current internal report shape

`SmokeEvalReport` in `crates/punk-eval/` remains the current canonical in-code shape for the local smoke harness.

This spec does not replace that internal Rust structure today.

Instead, it defines the bounded artifact shape that a later machine-readable output may map to.

## Assessment-only semantics

Smoke eval reports assess current behavior.

They do not:

- accept work;
- write acceptance or decision state;
- write gate output;
- produce proofpacks;
- become a final decision surface.

Command exit codes remain run results.

Final project acceptance remains a future `gate` responsibility.

## Proposed top-level shape

| Field | Required in v0.1 | Shape | Notes |
|---|---|---|---|
| `schema_version` | yes | string | Fixed version marker for the artifact shape, for example `smoke-eval-report.v0.1`. |
| `suite_id` | yes | string | Stable suite identifier, for example `smoke.v0`. |
| `run_id` | no | string | Deferred from required status until a later bounded goal defines run identity without implying `.punk/evals` storage. |
| `smoke_result` | yes | enum | Overall suite result. Default vocabulary is `pass` or `fail`. |
| `mode` | yes | string | Current execution mode, such as `local-smoke-check`. |
| `runtime_persistence` | yes | string | Boundary note for runtime state, for example `inactive`. |
| `report_storage` | yes | string | Boundary note for report storage, for example `inactive`. |
| `case_results` | yes | array | Ordered list of case-level results. |
| `boundary_notes` | yes | array of strings | Human-readable notes that preserve trust-surface boundaries. |
| `deferred` | yes | array of strings | Explicitly deferred capabilities or follow-up concerns. |
| `provenance` | no | object | Optional future provenance block. Deferred from the required set in v0.1. |

## Proposed case result shape

| Field | Required in v0.1 | Shape | Notes |
|---|---|---|---|
| `case_id` | yes | string | Stable case identifier across runs. |
| `status` | yes | enum | Default vocabulary is `pass` or `fail`. |
| `summary` | yes | string | Short human-readable result summary. |
| `assessment` | yes | string | Case-level assessment text. |
| `evidence_refs` | no | array of strings | Optional repo-relative evidence refs for future richer reports. |

## Default result vocabulary

Top-level `smoke_result` and case-level `status` use the same default vocabulary:

- `pass`
- `fail`

`skip` is intentionally omitted from v0.1.

Rationale:

- the current deterministic smoke harness runs a fixed bounded suite;
- current report semantics do not include waiver or partial execution states;
- adding `skip` now would imply additional lifecycle or policy semantics before they are defined.

If later work requires skipped or waived cases, that change should happen in a separate schema revision.

## Optional future provenance fields

A future schema revision may add a bounded provenance block such as:

- `produced_at`;
- `observed_at`;
- `runner_version`;
- `repo_ref`;
- `correlation_refs`.

These fields are deferred because current local smoke eval does not yet define stable run identity, stored runs, or report history.

## Non-goals

This v0.1 proposal does not define:

- final decisions;
- proofpack artifacts;
- gate output;
- baseline comparison;
- waiver ledger semantics;
- report persistence model;
- `.punk/evals` storage layout;
- JUnit, SARIF, TAP, or OTel export mappings.

## Boundary notes for future implementation

Any later implementation of machine-readable output must keep the following explicit:

- the schema is versioned;
- the artifact is assessment-only;
- storage is not implied by output shape alone;
- CLI JSON output is a separate decision from schema definition;
- future baseline and waiver behavior require separate bounded work.

## Next bounded step

If implementation proceeds, the next honest step is a narrow JSON output goal that maps the existing internal `SmokeEvalReport` to this proposed v0.1 shape without introducing storage, baseline, waiver, or validator behavior.
