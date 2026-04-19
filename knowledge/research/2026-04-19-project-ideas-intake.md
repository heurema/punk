---
id: research_2026_04_19_project_ideas_intake
title: "Project ideas intake for punk"
kind: research
status: accepted
authority: advisory
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [architecture, telemetry, proofpack, project-memory, eval]
related_goals: []
related_files:
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/TELEMETRY.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/adr/ADR-0011-local-first-trust-telemetry.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
source_refs:
  - https://www.inkandswitch.com/essay/local-first/
  - https://jj-vcs.github.io/jj/latest/operation-log/
  - https://www.openpolicyagent.org/docs/management-decision-logs
  - https://slsa.dev/provenance/
  - https://docs.deno.com/runtime/manual/getting_started/permissions
confidence: medium
research_level: R2
---

# Research intake: project mechanisms to adopt, save, or park

## Question

Which mechanisms from adjacent systems should `punk` adopt into active-core now, which should be deferred, and which should stay parked?

## Decision context

`punk` is explicitly core-first: project identity, flow state machine, append-only event log, local eval harness, contract lifecycle, gate decision, proofpack, inspectable state, and project memory are the current trust surfaces.

The goal of this intake is not to copy product surfaces. The goal is to extract mechanisms that strengthen local trust evidence without introducing a second source of truth, hidden network behavior, or premature agent/platform scope.

## Sources reviewed

| Source | Tier | Why included | Key limitation |
|---|---|---|---|
| Ink & Switch, "Local-first software" | A | Strong fit for ownership, local-first storage, and anti-cloud-primary posture | Broad philosophy, not a Punk-specific runtime design |
| Jujutsu operation log docs | A | Clear operation-log, replay, and undo/view model | VCS-specific semantics need translation |
| OPA decision logs | A | Good model for assessable decisions, decision ids, and redaction needs | Policy engines can overtake decision ownership if mapped naively |
| SLSA provenance guidance | A | Useful minimal provenance framing and artifact-hash discipline | Supply-chain scope is broader than current Punk needs |
| Deno permissions docs | A | Clear default-deny permission model | Runtime permissions are not yet a concrete Punk subsystem |

## Existing systems / prior art

### Local-first is the right trust posture

Ink & Switch's local-first framing is a strong match for Punk's direction: the user owns the data, local storage matters, and the server should not become the primary source of truth.

Fit for `punk`:

- local-first trust evidence
- no hidden telemetry export
- explicit later grants for side effects

### Jujutsu shows why replayable history matters

Jujutsu's operation log demonstrates the value of storing operations and replaying state from canonical history.

Fit for `punk`:

- `.punk/events/*.jsonl` as append-only local evidence
- replay-backed `inspect`
- derived views under `.punk/views/` as convenience only

### OPA shows the right boundary between assessment and decision

OPA decision logging is useful not because `punk` should adopt Rego now, but because it separates assessment from enforcement and emphasizes auditability and masking.

Fit for `punk`:

- guard receipts with reason codes
- redaction layer
- policy/module assessment without policy ownership of the final decision

### SLSA gives the right minimum provenance shape

SLSA provenance is larger than current scope, but its primitives are useful now: what artifact was produced, by what process, with what references and hashes.

Fit for `punk`:

- minimal proofpack manifest
- contract/run/eval/decision linkage
- artifact hashes and event-range references

### Deno reinforces default-deny side effects

Deno's permission model is a good reminder that file, network, env, and subprocess access should never be implicit.

Fit for `punk`:

- no-network default
- future sandbox grant receipts
- explicit permission scope as future work

## Failure modes found

- copying product surfaces instead of extracting mechanisms
- remote telemetry becoming a hidden side effect
- policy engines or modules implying final decisions
- proofpacks becoming narrative-only artifacts without hashes
- derived views becoming a second source of truth
- memory turning into one giant prompt instead of linked artifacts
- pulling sandbox, DAG, cache, agent, and marketplace scope into active-core too early

## Options considered

### Option A — Adopt only existing repo plans and ignore new research

Pros:

- minimal churn
- no new docs

Cons:

- misses strong trust-evidence improvements
- leaves telemetry/provenance/privacy under-specified

### Option B — Adopt mechanisms, not products

Pros:

- aligns with Punk laws
- strengthens active-core trust surfaces
- keeps roadmap focused

Cons:

- requires more documentation up front
- creates follow-up pressure for evals and schemas

### Option C — Pull in future ideas now

Pros:

- feels ambitious quickly

Cons:

- breaks core-first discipline
- increases risk of hidden truth surfaces and scope drift

## Recommendation

Adopt Option B.

## Adopt into active-core

| Mechanism | Source pattern | Punk mapping | Why adopt now | Required docs/evals |
|---|---|---|---|---|
| Operation/event ledger | Jujutsu operation log | `.punk/events/*.jsonl`, replay state | Flow needs inspectability from day zero | `TELEMETRY.md`, replay eval |
| Guard denial receipts | OPA decision logs | guard result event with reason code | Illegal transitions must be explainable | guard denial eval |
| Minimal proofpack | SLSA provenance | proof manifest linking contract/run/eval/decision hashes | Gate decisions need evidence | proofpack ADR/evals |
| Artifact hashes | SLSA/Nix-style discipline | hash refs for contracts, receipts, reports | Avoid narrative-only proof | artifact-hash eval |
| Deterministic smoke evals | local harnesses | local eval JSON + Markdown report | Phase promotion needs proof | eval spec |
| No-network default | Deno permissions, local-first principles | external side effects stay explicit | Protect privacy and trust | no-network eval |
| Redaction layer | OPA masking guidance | telemetry and receipt writer rejects raw sensitive content | Prevent prompt/source/secret leakage | redaction eval |
| Derived inspect views | replay/view models | `.punk/views/` generated from canonical evidence | Useful UI without second truth | inspect consistency eval |
| Project-memory link graph | ADR/RFC/project-memory systems | goal -> contract -> report -> decision -> proof links | Avoid giant-prompt memory | memory-link eval |

## Defer

| Mechanism | Why defer | Trigger |
|---|---|---|
| Full DAG scheduler | Useful, but can swallow core lifecycle too early | after flow, contract, gate, and proof are stable |
| Action cache | Needs hash discipline first | after proofpack hash model exists |
| Rich policy language | Premature before guard model is stable | after at least five real guard cases |
| OpenTelemetry export | External side effect and privacy boundary | after local telemetry + redaction evals pass |
| Remote transparency log | Adds external trust dependency | after local proofpack is stable |
| UI views | May hide source-of-truth rules | after inspect CLI is stable |
| Agent execution | High trust risk | after sandbox, permissions, eval gate, and proof are stable |

## Park

- SaaS control plane
- marketplace
- public registry
- remote orchestration
- team collaboration and sync
- cloud dashboards
- hidden analytics

## Avoid

- AI decides final acceptance
- modules write decisions
- adapters own truth
- hidden network calls
- raw prompt or source telemetry
- proof without artifact hashes
- evals without fixtures
- memory as one giant prompt
- dashboards not backed by local canonical state
- plugin ecosystem before permission model

## What stays out of scope

- remote telemetry export
- transparency-log integration
- cloud sync
- marketplace and plugin ecosystem
- autonomous agent execution
- rich permission DSL before minimal receipts

## Impact on roadmap

- Phase 1 should define the first event schema fixture, replay expectations, redaction rules, and no-network default checks.
- Phase 2 should ensure evals emit both machine JSON and human Markdown with artifact refs.
- Phase 3 should make proofpack provenance inspectable and gate-owned.
- Phase 4 should add project-memory links and derived drift views without turning memory into hidden truth.

## Required evals

- telemetry event schema fixture
- denied transition receipt
- replay reconstructs flow state
- no-network default
- redaction of paths, secrets, and raw content
- proofpack hash integrity
- only gate writes final decision

## Required docs / ADRs / contracts

- `docs/product/RESEARCH-INTAKE.md`
- `docs/product/TELEMETRY.md`
- `docs/adr/ADR-0011-local-first-trust-telemetry.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/2026-04-19-core-trust-evals.md`

## Open questions

- Should the canonical event type be `PunkEvent`, `AuditEvent`, or `LedgerEvent`?
- Should event ids be ULID, UUIDv7, or content-addressed hashes?
- Does `punk-proof` need to be its own stable boundary immediately, or can proofpack stay closer to `punk-gate` at first?
- Should `punk inspect telemetry` appear in Phase 2 or later?
