# Eval baseline and waiver boundary v0.1

Date: 2026-04-22
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Punk baseline and waiver behavior must satisfy before any baseline comparison or waiver implementation begins.

This is a design/spec artifact only.

It clarifies how future baselines and waivers may later be introduced without turning eval into hidden authority or weakening `gate` / proof boundaries.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate `.punk/evals`.

It does not define baseline comparison code or waiver ledger files.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and may require additional evals and an ADR if baseline or waiver semantics become project-policy-level.

## Relationship to adjacent artifacts

### Relationship to `smoke-eval-report.v0.1`

`evals/specs/smoke-eval-report.v0.1.md` defines the proposed smoke eval report shape.

This boundary spec does not replace that schema.

Instead, it defines the conditions under which future baseline and waiver artifacts may later reference reports shaped like that schema.

### Relationship to `eval-storage-boundary.v0.1`

`evals/specs/eval-storage-boundary.v0.1.md` defines the storage constraints for any future stored eval artifacts.

This boundary spec builds on those storage constraints.

Baseline and waiver semantics must not bypass append-only, run-scoped, evidence-only storage rules.

### Relationship to future `.punk/evals`

`.punk/evals` is still future runtime/derived storage.

This boundary spec does not activate it.

If baseline or waiver artifacts are introduced later, they must respect the same canonical-vs-derived boundary as stored eval reports.

### Relationship to future gate/proof

Future `gate` or proof steps may later reference baseline selections, waiver records, or related report hashes.

Baseline and waiver artifacts must not themselves accept work, approve work, or replace final decision state.

## Baseline principles

Future baseline behavior must satisfy all of the following:

1. **Baseline is future, not active now.**
   No baseline comparison behavior is implied by this spec.

2. **Baseline is an explicitly selected report or artifact.**
   A baseline must point to a chosen report or report artifact rather than an implicit moving target.

3. **Baseline must not mean mutable `latest`.**
   The newest available report is not automatically the baseline.

4. **Baseline comparison is assessment, not acceptance.**
   A baseline comparison may describe deltas or regressions, but it must not accept work.

5. **Baseline pass does not imply gate acceptance.**
   Passing a baseline comparison cannot replace `gate`.

6. **Baseline should later record reference identity.**
   Future baseline artifacts should capture report ref, schema/version, and hash identity explicitly.

## Minimum future baseline metadata

The first future baseline artifact should carry at least:

- `baseline_id`;
- `suite_id`;
- `baseline_report_ref`;
- `baseline_report_hash` or artifact-hash concept;
- `selected_by`;
- `selected_at`;
- `reason`;
- `scope`.

A future revision may also carry:

- `expires_at`;
- `review_after`.

These fields exist so that baseline selection stays explicit, inspectable, and revisitable.

## Waiver principles

Future waiver behavior must satisfy all of the following:

1. **Waiver is future, not active now.**
   No waiver behavior is implied by this spec.

2. **Waiver is explicit, scoped, reasoned, owner/approver-tagged, and revisitable.**
   A waiver must be a deliberate record rather than an inline ignore.

3. **Waiver must not silently hide failures.**
   Future waived failures must remain inspectable.

4. **Waiver must not weaken Punk Laws.**
   It cannot convert assessment output into final authority.

5. **Waiver does not turn failing evals into acceptance.**
   A waiver may explain why a failure does not currently block promotion, but it does not become a final decision by itself.

6. **Waiver should later be ledgered and referenceable.**
   Future gate/proof may cite waiver ids and related report hashes as evidence only.

## Minimum future waiver metadata

The first future waiver artifact should carry at least:

- `waiver_id`;
- `suite_id`;
- `case_id` or explicit scope;
- `reason`;
- `owner`;
- `approver`;
- `created_at`;
- `expires_at` or `review_after`;
- `related_report_ref`;
- `related_report_hash` or artifact-hash concept.

These fields exist so that waivers remain explicit, bounded, and auditable later.

## Non-goals

This v0.1 boundary does not define:

- implementation;
- `.punk/evals` creation;
- file writes;
- report persistence behavior;
- baseline comparison implementation;
- waiver ledger implementation;
- validators;
- gate/proof implementation;
- remote storage;
- dashboarding or coherence scoring.

## Future implementation constraints

Any later implementation step should be explicit about:

- baseline reference selection rules;
- deterministic comparison surface;
- waiver scope and expiry semantics;
- report-hash or artifact-hash requirements;
- how waived or baseline-compared results remain inspectable without becoming hidden truth;
- how future gate/proof references baseline or waiver records only as evidence.

## Why baseline and waiver are not authority

Baselines and waivers are governance/evidence surfaces.

They are not final authority because:

- baseline selection is a comparison input, not a decision output;
- baseline pass can coexist with other unresolved decision criteria;
- waiver explains why a specific failure is temporarily tolerated, but it does not accept work;
- only `gate` writes the final decision.

## Deferred for later specs or goals

Still deferred after this boundary spec:

- baseline comparison algorithm;
- waiver artifact format implementation;
- `.punk/evals` layout implementation;
- validator rules for baseline or waiver records;
- gate/proof reference mechanics beyond the requirement that they remain evidence-only.

## Next bounded step

If the repo needs another conservative process step before implementation, run a second advisory Work Ledger Review over the recent schema/storage/baseline boundary sequence.
