---
id: docs_product_research_gate
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - research-gate-policy
  - research-depth-levels
  - research-lifecycle
  - research-promotion-rule
  - research-note-requirements
related_docs:
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/START-HERE.md
related_adrs:
  - docs/adr/ADR-0006-research-gate-before-major-decisions.md
supersedes: []
superseded_by: null
---
# Research Gate

## Purpose

Important architecture/product decisions must be informed by external evidence before implementation.

The Research Gate prevents design-by-inspiration and prevents the project from repeating known mistakes already discovered by adjacent tools and papers.

## Principle

Research is required for important decisions, not for every tiny implementation detail.

A decision needs a research note when it changes one of:

- core laws
- flow semantics
- eval policy
- storage model
- module interface
- adapter boundary
- project memory model
- knowledge/retrieval behavior
- external side-effect policy
- roadmap phase boundary
- public CLI contract

Minor code edits, formatting, straightforward bug fixes, and implementation of an already-researched contract do not need a new research note.

## Research lifecycle

```text
Question
  -> Source collection
  -> Source quality rating
  -> Pattern extraction
  -> Failure-mode extraction
  -> Recommendation
  -> ADR / Goal / Contract refs
```

## Research intake classification

Every non-trivial external idea must be classified as exactly one of:

- `adopt` — belongs in active-core now
- `defer` — valuable, but not needed for the current phase
- `park` — out of scope until the roadmap explicitly promotes it
- `avoid` — conflicts with Punk laws or trust posture

See `docs/product/RESEARCH-INTAKE.md`.

## Storage

Research notes are repo-tracked:

```text
knowledge/research/
  YYYY-MM-DD-short-topic.md
```

They may link to raw notes, but the accepted synthesis must be concise and curated.

## Source quality tiers

### Tier A — primary / authoritative

Examples:

- official docs
- official repositories
- standards/specifications
- peer-reviewed or arXiv papers with clear methodology
- maintained benchmark docs
- vendor docs for the vendor's own system

### Tier B — credible field evidence

Examples:

- mature project docs
- issue trackers and PR discussions from relevant projects
- engineering blogs with concrete implementation detail
- postmortems
- reproducible benchmark reports

### Tier C — weak / exploratory

Examples:

- personal opinions
- unsourced blog posts
- anonymous comments
- promotional listicles
- unverified claims

Tier C can inspire questions. It should not justify a core decision alone.

## Required sections in a research note

- Question
- Decision context
- Sources reviewed
- Source quality table
- Existing systems / prior art
- Failure modes found
- Options considered
- Recommendation
- Adoption map: adopt / defer / park / avoid
- What stays out of scope
- Impact on roadmap
- Required evals
- Required docs/ADRs/contracts
- Open questions

## Research depth levels

### R0 — no research required

Use for:

- small implementation detail
- typo/docs cleanup
- adding tests for known behavior
- executing an already-approved contract

### R1 — quick scan

Use for bounded decisions with limited blast radius.

Expected output:

- 3-5 sources
- one short research note
- clear recommendation

### R2 — design research

Use for architecture-affecting decisions.

Expected output:

- 8-15 curated sources
- prior-art comparison
- failure-mode extraction
- ADR-ready recommendation
- eval implications

### R3 — deep research

Use for major product direction or irreversible architecture.

Expected output:

- broad source review
- multiple alternatives
- explicit trade-off matrix
- risk register updates
- roadmap updates

## Promotion rule

Research does not automatically become product truth.

Promotion path:

```text
research note
  -> ADR or roadmap decision
  -> goal/contract
  -> implementation
  -> eval result
  -> project knowledge update
```

## Contract rule

Contracts for important architecture work must include:

- `research_refs`
- `decision_refs`
- `eval_refs`
- explicit no-research rationale if no research was needed

## Anti-patterns

Do not:

- collect everything
- trust low-quality commentary as evidence
- let research block every small step
- treat benchmark leaderboards as product truth
- copy another project's architecture without mapping gaps
- keep research only in chat
- skip eval implications
- skip failure modes

## Research intake triggers

A Research Gate is required before adopting ideas that change:

- event-log shape or replay model
- gate/proof boundaries
- telemetry storage or export behavior
- redaction/privacy model
- project-memory link model
- sandbox/permission semantics
- public claims about trust evidence
- active-core scope or parked/future boundaries

## Knowledge Vault research requirements

Changes to Knowledge Vault storage, retrieval, indexing, promotion, or external sharing require a research note unless they are narrow implementations of an already accepted ADR or contract.

Because these changes affect project memory and decision boundaries, they will usually require R2 or R3 depth.

A Knowledge Vault research note must include:

- source quality table
- prior-art comparison
- failure modes
- authority/trust model implications
- local-first implications
- privacy/security implications
- eval requirements
- adoption map: adopt / defer / park / avoid
- explicit out-of-scope list

Research output remains advisory until promoted through ADR, roadmap, contract, implementation, eval, and proof.
