# Surgical change discipline eval spec v0.1

Date: 2026-04-25
Status: docs/eval-policy spec; not runtime-enforced yet
Authority: advisory until implemented by a future eval runner or gate policy

## Purpose

Define policy checks for keeping Punk work bounded to the approved contract. The goal is to detect drive-by refactors, speculative scope, provider coupling, and unverified executor claims before they weaken gate decisions or proofpacks.

This spec documents expected review behavior. It does not accept work and does not currently enforce runtime behavior.

## Scope

The spec applies to docs, examples, eval-policy artifacts, and future code changes when a contract declares allowed paths, protected paths, non-goals, validators, proof requirements, or public-claim boundaries.

It covers executor discipline, not executor identity. A human, script, model, agent, IDE, module, or adapter can violate or satisfy the same discipline.

## Active-core vs future status

Active now:

- use this spec as manual review and docs/eval-policy guidance;
- cite it from reports when scope discipline is relevant;
- keep executor claims non-proof and semantic assessment advisory.

Future or deferred:

- line-level traceability enforcement;
- formal receipt schema fields for changed-file traceability;
- semantic assessor command interface;
- automatic task-scoped executor brief generation;
- hard-gate integration in an eval runner or gate policy.

## Deterministic checks

A future deterministic implementation should check:

- changed files fit the contract allowed paths;
- protected paths are not touched;
- no new dependency appears without an explicit contract clause;
- no provider/model-specific coupling appears without approval;
- no public docs claim a parked/future capability is active;
- no runtime/code/schema files changed in a docs-only contract;
- no unrelated formatting churn dominates the diff;
- every changed file traces to a goal, non-goal exception, acceptance criterion, or proof requirement.

## Semantic / advisory checks

A semantic assessor may advise on:

- whether a change is simpler than plausible alternatives;
- whether abstractions are speculative;
- whether a refactor is necessary for the accepted goal;
- whether docs duplicate canonical policy instead of cross-referencing it;
- whether a public claim is stronger than evidence supports;
- whether unavailable checks or executor claims are clearly marked.

Semantic assessor output is advisory evidence only. It must not become the gate decision.

## Failure examples

- A docs-only task edits `crates/`.
- A typo fix rewrites a full page.
- A narrow validator addition adds a new dependency without contract approval.
- A future provider name appears in active-core architecture without Research Gate promotion.
- A report says "tests passed" but includes no command or artifact output.
- A semantic assessor declares final acceptance.
- A public page says Punk executes coding-agent work today.

## Pass / fail / warn criteria

Pass:

- all changed files are in allowed scope;
- protected paths are untouched;
- no unapproved dependency, provider coupling, runtime behavior, schema field, or CLI surface appears;
- claims are backed by validator output or explicitly marked unverified;
- gate remains the final decision authority.

Fail:

- protected paths are changed without contract amendment;
- executor claims are treated as proof;
- semantic assessment is treated as final acceptance;
- a parked/future capability is described as active;
- a dependency, provider adapter, CLI command, runtime behavior, or schema change appears outside contract scope.

Warn:

- changed files are allowed but weakly tied to acceptance criteria;
- formatting churn makes review harder;
- a check is unavailable and no policy says whether to accept, reject, or defer;
- examples are provider-specific when a provider-neutral form would work;
- docs repeat canonical policy instead of linking to it.

## Evidence expected

A complete review should include:

- contract or goal reference;
- changed-file list;
- allowed/protected path comparison;
- validator commands and exact output;
- unavailable checks, if any;
- semantic advisory findings, if used;
- gate decision reference when gate exists;
- proofpack reference when proof exists;
- project-memory links for durable findings.
