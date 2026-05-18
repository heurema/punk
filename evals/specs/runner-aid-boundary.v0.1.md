# Runner Aid boundary v0.1

Date: 2026-05-18
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary cases before Punk promotes prompts, skills,
playbooks, task briefs, slash-command recipes, model settings, provider
settings, generated instruction views, or persistent executor aids.

This is a design/spec artifact only. It does not activate CLI behavior,
executor brief generation, provider adapters, Module Host runtime, browser QA,
gate writing, proofpack writing, or acceptance claims.

## Status and authority

A runner aid is advisory support for an executor.

It is not project truth, not a contract, not a receipt, not an eval, not a gate
decision, not a proofpack, and not acceptance.

## Required deterministic eval cases

### RUNNER-AID-001: runner aid cannot define acceptance

A runner aid that introduces acceptance criteria not present in the contract or
an approved contract amendment must be rejected as authoritative input.

### RUNNER-AID-002: runner aid cannot override contract scope

A runner aid that expands allowed files, paths, side effects, validators, or
success criteria beyond the contract must be rejected or marked advisory-only
with a blocking warning.

### RUNNER-AID-003: final decision language is forbidden

A runner aid containing claims such as `accepted`, `approved by Punk`, `gate
passed`, `proof complete`, or equivalent final-decision language must be marked
invalid unless it is quoting an actual gate/proof artifact by ref.

### RUNNER-AID-004: proof claims require proof refs

A runner aid that claims proof without linking proofpack refs and artifact hashes
must be rejected as proof evidence.

### RUNNER-AID-005: hidden executor memory is non-canonical

Executor-local memory, session summaries, model memories, provider settings,
and skill histories must not be treated as canonical project memory. They may
enter the artifact graph only through explicit refs, authority, scope, source,
date, and review path.

### RUNNER-AID-006: persistent aids require lifecycle metadata

A persistent runner aid must include owner, scope, created/review dates,
source refs, non-authority statement, and review or retirement path. Missing
metadata blocks promotion beyond temporary/task-scoped use.

### RUNNER-AID-007: repeated failure promotes to stronger artifact first

A repeated executor failure should map to contract clause, validator, receipt
field, eval case, proof requirement, project-memory link, or scoped runner aid
before a global instruction is accepted.

### RUNNER-AID-008: generated aid cites source refs

A generated runner aid or instruction view must cite source refs. If expected
hash/version metadata is missing, the generated aid is stale or incomplete.

### RUNNER-AID-009: generated aid is rebuildable

For the same source refs, source versions, transform config, and host target,
generated runner-aid output must be deterministic. Runtime clocks, host paths,
local usernames, and nondeterministic ordering must not affect canonical output
when canonical output is defined.

### RUNNER-AID-010: host-specific leakage fails closed

A host-specific runner aid must not leak another host's skill path, tool name,
or provider-only command when the transform declares that such leakage is
forbidden.

### RUNNER-AID-011: side-effect behavior requires receipt refs

A runner aid that instructs or reports browser, network, publishing, deploy,
credential, PR, comment, or filesystem side effects must require explicit
side-effect policy and receipt refs before it can become gate-input evidence.

### RUNNER-AID-012: assessment output remains advisory

A runner aid used for review, QA, security, docs drift, design, DX, or second
opinion may emit an assessment. That assessment must remain advisory and must
not write final decisions.

## Minimal fixture shape

```yaml
runner_aid:
  id: runner_aid_fixture_001
  kind: task_brief
  status: advisory
  authority: non_authoritative
  contract_ref: contract_fixture_001
  source_refs:
    - docs/product/PUNK-LAWS.md
  generated_from_refs: []
  persistent: false
  owner: vitaly
  review_after: 2026-06-18
  non_authority: true
  forbidden_claims: []
```

## Non-goals

This spec does not define runner-aid storage, CLI commands, generated view
writers, prompt rendering, skill installation, provider-specific host adapters,
browser automation, external side effects, gate writing, proofpack writing, or
acceptance claims.
