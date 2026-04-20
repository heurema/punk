---
id: report_2026_04_19_documentation_consolidation
kind: report
status: draft
authority: advisory
created_at: 2026-04-19
updated_at: 2026-04-19
related_docs:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/TELEMETRY.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - docs/adr/ADR-0011-local-first-trust-telemetry.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
---

# Documentation consolidation report

## Question

How should the current Punk documentation be organized so the project has one coherent system of record without overlapping or conflicting truth?

## Current state

The docs are directionally coherent and already enforce the right product posture:

- core-first
- local-first trust evidence
- `plot / cut / gate`
- only `gate` writes final decisions
- proof before acceptance
- research before important decisions
- project memory from day zero
- public build from day zero
- modules assess, adapters invoke, neither owns truth

The main issue is not contradiction. The main issue is repeated explanations across entry docs, architecture docs, roadmap docs, research docs, telemetry docs, and ADRs.

## Main overlaps found

| Topic | Appears in | Canonical owner should be |
|---|---|---|
| Punk identity / not production-ready | `README.md`, `START-HERE.md`, public narrative | `README.md` + `START-HERE.md` short summary |
| Active-core vs not-active | `README.md`, `START-HERE.md`, `ROADMAP.md`, `CRATE-STATUS.md` | `ROADMAP.md` + `CRATE-STATUS.md` |
| Research before decisions | `README.md`, `START-HERE.md`, `PUNK-LAWS.md`, `RESEARCH-GATE.md`, `RESEARCH-INTAKE.md` | `RESEARCH-GATE.md` for requirement, `RESEARCH-INTAKE.md` for idea classification |
| Local trust telemetry | `README.md`, `ARCHITECTURE.md`, `RESEARCH-INTAKE.md`, `TELEMETRY.md`, ADR-0011 | `TELEMETRY.md` + ADR-0011 |
| Proofpack provenance | `ARCHITECTURE.md`, `ROADMAP.md`, `TELEMETRY.md`, ADR-0012 | `ARCHITECTURE.md` + ADR-0012 |
| Assessment vs decision | `PUNK-LAWS.md`, `ARCHITECTURE.md`, ADR-0013 | `PUNK-LAWS.md` + ADR-0013 |
| Project memory surfaces | `README.md`, `START-HERE.md`, `ARCHITECTURE.md`, `PROJECT-MEMORY.md` | `PROJECT-MEMORY.md` |
| Public narrative | `README.md`, `START-HERE.md`, `PUBLIC-NARRATIVE.md` | `PUBLIC-NARRATIVE.md` |

## Recommended consolidation

### Add now

1. `docs/product/DOCUMENTATION-MAP.md`
2. `docs/product/GLOSSARY.md`
3. `evals/specs/2026-04-19-doc-consistency-evals.md`

### Update next

1. Shorten `README.md` to public identity + current target + links.
2. Shorten `START-HERE.md` to internal orientation + reading order + active operator boundary.
3. Keep `ARCHITECTURE.md` as structural boundaries, not research rationale.
4. Keep `ROADMAP.md` as phase gates, not schema definitions.
5. Keep `RESEARCH-GATE.md` as when/how research is required.
6. Keep `RESEARCH-INTAKE.md` as adopt/defer/park/avoid process.
7. Keep `TELEMETRY.md` as event schema, redaction, no-network default, and telemetry evals.
8. Keep ADRs as decision rationale and consequences.

## Proposed source-of-truth rule

Every future doc edit should answer:

1. Which document owns this truth?
2. Does this edit change active-core behavior or only summarize it?
3. Does this edit imply a parked capability is active?
4. Does this edit require Research Gate?
5. Does this edit need an eval case?

## Adopt / defer / park / avoid

| Item | Classification | Why |
|---|---|---|
| Documentation map | adopt | Reduces drift and makes ownership explicit |
| Glossary | adopt | Prevents duplicated term definitions |
| Docs consistency eval | adopt/defer | Strong fit, but implementation can follow active eval harness maturity |
| Duplicate-section detector | defer | Useful after docs map/glossary exist |
| Generated docs site | park | UI/public surface too early |
| LLM doc judge as hard gate | avoid for now | Non-deterministic and can create opaque acceptance |

## What stays out of scope

- rewriting laws
- changing roadmap phases
- promoting adapters/modules
- adding competitor ideas to active-core
- treating research/ideas as implementation truth
- adding public docs automation

## Next actions

1. Add `DOCUMENTATION-MAP.md`.
2. Add `GLOSSARY.md`.
3. Add docs consistency eval spec.
4. Review proposed `README.md` and `START-HERE.md` replacements.
5. Apply only after checking that no active/parked status changed unintentionally.

