---
id: docs_product_telemetry
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - local-first-trust-telemetry
  - event-schema-v0.1
  - telemetry-redaction-rules
  - no-network-default
  - telemetry-export-boundary
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
related_adrs:
  - docs/adr/ADR-0011-local-first-trust-telemetry.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
related_evals:
  - evals/specs/2026-04-19-core-trust-evals.md
supersedes: []
superseded_by: null
---
# Telemetry

Status: accepted design direction for local trust telemetry; remote export is parked
Last updated: 2026-04-19

Decision refs:

- `docs/adr/ADR-0011-local-first-trust-telemetry.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`

## Purpose

Telemetry in `punk` is local-first trust evidence, not hidden analytics.

The first telemetry surface exists to answer:

- what happened?
- which command attempted it?
- which guard allowed or denied it?
- which contract, run, eval, receipt, decision, and proof were involved?
- can inspect/replay reconstruct the state?
- can the user verify that no hidden network export happened?

## Non-goals

Telemetry is not:

- product analytics
- user tracking
- remote reporting
- a SaaS control plane
- a second project memory
- a replacement for gate decisions
- a replacement for proofpacks
- a module decision channel

## Status split

| Surface | Status | Storage | Source of truth |
|---|---|---|---|
| core audit events | active-core | `.punk/events/*.jsonl` | yes, for flow transitions |
| eval receipts | active-core | `.punk/evals/`, `work/reports/` | yes, for eval runs |
| run receipts | active-core | `.punk/runs/` | yes, for run evidence |
| gate decisions | active-core | `.punk/decisions/` | yes, for final decision |
| proofpack manifests | active-core | `.punk/proofs/` | yes, for proof references |
| derived inspect views | active-core later | `.punk/views/` | no, regenerable |
| remote export | parked | none | no |

## Laws

1. Core audit events are local and required for trust surfaces.
2. Remote export is disabled by default and parked.
3. Telemetry never decides; only `gate` writes final decisions.
4. Telemetry events may reference artifacts but must not duplicate canonical artifact content.
5. Derived views are always regenerable from events, receipts, decisions, and proofs.
6. Raw user content, source snippets, prompts, secrets, env vars, absolute home paths, hostnames, and git remotes must not be stored in telemetry by default.
7. Every exported or public telemetry surface requires explicit review, Research Gate, ADR, redaction evals, and opt-in semantics.

## Event schema v0.1

```yaml
schema_version: punk.event.v0.1
event_id: evt_...
emitted_at: 2026-04-19T00:00:00Z
source:
  crate: punk-flow
  component: transition_guard
phase: plot|cut|gate|eval|inspect|init
kind: transition_attempt|transition_committed|transition_denied|guard_evaluated|eval_run|receipt_written|decision_written|proof_written|error
severity: trace|debug|info|warn|error
correlation:
  project_id: local-project-id-or-hash
  flow_id: flow_...
  goal_ref: work/goals/...
  contract_id: contract_...
  run_id: run_...
  eval_run_id: eval_...
  decision_id: decision_...
  proof_id: proof_...
result:
  status: started|succeeded|failed|denied|skipped
  guard_code: CUT_REQUIRES_APPROVED_CONTRACT
  error_code: E_FLOW_ILLEGAL_TRANSITION
  duration_ms: 123
artifacts:
  refs:
    - .punk/contracts/contract_....json
  hashes:
    - sha256:...
privacy:
  class: core_audit|minimal_diagnostic|derived
  redacted: true
  contains_raw_user_content: false
  exportable: false
```

## Minimal event kinds

Phase 1 must support:

- `project_initialized`
- `transition_attempt`
- `transition_committed`
- `transition_denied`
- `guard_evaluated`
- `inspect_read`
- `event_replay_started`
- `event_replay_completed`

Phase 2 must add:

- `eval_run_started`
- `eval_case_started`
- `eval_case_finished`
- `eval_run_finished`
- `eval_report_written`

Phase 3 must add:

- `contract_accepted`
- `run_receipt_written`
- `gate_decision_written`
- `proofpack_written`

Phase 4 must add:

- `memory_link_created`
- `memory_drift_detected`
- `derived_view_written`

## Redaction rules

Telemetry writers must reject or redact:

- absolute paths under user home
- environment variable values
- tokens and secret-looking values
- raw source file contents
- raw prompt contents
- git remote URLs
- hostnames
- usernames
- IP addresses
- provider request/response bodies

Prefer:

- workspace-relative refs
- stable local ids
- hashes
- enum error codes
- explicit guard codes
- artifact references

## Implementation stance

Rust `tracing` may be used for internal diagnostics, spans, and debug visibility.

Canonical Punk telemetry must be written through an explicit `punk-events` writer. Do not persist arbitrary `tracing` fields as canonical audit events.

This keeps diagnostic instrumentation from accidentally becoming project truth or leaking sensitive context.

## No-network default

Default Punk runs must perform no telemetry network export.

A future export adapter requires:

- Research Gate
- ADR
- opt-in config
- local preview of exported payload
- redaction evals
- no-network default eval
- consent receipt
- visible disable path

## Required evals

- `telemetry_event_schema_fixture`
- `flow_denied_cut_before_approved_contract`
- `eval_machine_and_human_reports`
- `replay_reconstructs_flow_state`
- `proofpack_hashes_match`
- `telemetry_no_network_default`
- `telemetry_redaction_paths_and_secrets`
- `only_gate_writes_decision`
- `proofpack_requires_decision`

See `evals/specs/2026-04-19-core-trust-evals.md`.

## Open questions

- Should the canonical type be called `PunkEvent`, `AuditEvent`, or `LedgerEvent`?
- Should event ids be ULID, UUIDv7, or content-addressed hash?
- Should project id be a stable local id, explicit project identity, or hash-based?
- What is the event-log rotation policy?
- How much sanitized error text is allowed beyond error codes?
