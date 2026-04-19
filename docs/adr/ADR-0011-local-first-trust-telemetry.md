# ADR-0011: Local-first trust telemetry

Status: Accepted
Date: 2026-04-19

Research refs:

- `knowledge/research/2026-04-19-project-ideas-intake.md`

## Context

`punk` needs trust evidence from day zero. The core workflow depends on knowing which transitions happened, which guards allowed or denied them, which evals ran, and which decision/proof artifacts were written.

Remote telemetry would introduce hidden side effects, privacy risk, and a premature external boundary. It is not required to prove local core behavior.

## Decision

Adopt local-first trust telemetry as active-core.

Remote telemetry export is parked.

The canonical telemetry surface is an append-only local audit event ledger plus receipts, decisions, proofs, and derived inspect views.

Telemetry does not decide. `gate` decides.

Telemetry can reference canonical artifacts but must not duplicate their raw content.

## Consequences

Positive:

- flow state can be inspected and replayed
- illegal transitions can leave auditable denials
- eval/gate/proof evidence becomes linked
- project memory can derive summaries without becoming a giant prompt
- no hidden analytics or default network calls are introduced

Negative:

- event schema must be designed early
- redaction tests become mandatory
- derived views need replay discipline
- future export needs a separate ADR

## Implementation notes

- use `.punk/events/*.jsonl` for append-only core audit events
- use explicit event-writer APIs rather than persisting arbitrary diagnostic logs
- use Rust `tracing` for diagnostics only where useful
- include correlation ids for project, flow, contract, run, eval, decision, and proof
- add replay and redaction smoke evals before extending the schema

## Required evals

- telemetry event schema fixture
- denied transition event
- no-network default
- redaction of paths, secrets, and raw content
- replay reconstructs flow state

## Out of scope

- remote export
- OpenTelemetry collector integration
- dashboards
- product analytics
- SaaS control plane
