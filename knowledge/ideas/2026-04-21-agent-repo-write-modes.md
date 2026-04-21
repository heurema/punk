---
id: idea_2026_04_21_agent_repo_write_modes
title: "Agent repo write modes: PR review path vs direct-to-main YOLO mode"
kind: idea
status: raw
authority: speculative
owner: vitaly
created_at: 2026-04-21
updated_at: 2026-04-21
review_after: 2026-07-21
components: [agent-execution, dev-module, git, repo-write, review]
related_goals: []
related_files:
  - docs/product/ROADMAP.md
  - docs/product/PUNK-LAWS.md
  - docs/product/DOGFOODING.md
  - docs/product/MODULE-HOST.md
source_refs: []
confidence: medium
---

# Agent repo write modes: PR review path vs direct-to-main YOLO mode

## Summary

Punk may eventually need an explicit repo-write mode for agent-driven changes.

Preserve two future modes:

- reviewable PR mode
- high-trust direct-to-main YOLO mode

This is a parked idea, not current architecture and not an operator promise.

## Idea

When agent-driven repo modification exists, let the git write path be selected explicitly rather than implied.

Possible modes:

- `pull-request` — agent writes to a branch and opens a PR for human review.
- `direct-main` — agent writes directly to `main` or the active trunk with no PR.
- later, possibly safer non-side-effect modes such as `workspace-only` or `dry-run`.

Possible control surfaces:

- project-level default in config or policy
- contract-level override with explicit reason
- CLI flag such as `--write-mode pr|direct-main`

## Why preserve it

PR mode is useful for:

- human review and discussion
- visible diff and branch isolation
- stronger control on trust-surface changes

Direct-main / YOLO mode may be useful for:

- local-first solo workflows
- tight inner loops when the operator explicitly accepts the risk
- low-risk docs or sandbox changes
- environments where PRs add friction without adding meaningful review

## Non-negotiable constraints

Any future repo-write mode must preserve Punk laws and boundaries:

- one lifecycle remains `plot / cut / gate`
- module or agent output is not final truth by itself
- only `gate` writes final acceptance
- direct-to-main must not bypass receipts, evals, decisions, or proof
- trust-surface changes still require stronger review
- overrides must be explicit and ledgered
- project memory must preserve who changed what, why, and under which mode

## Why parked now

- autonomous coding agent execution is not active-core
- git side-effect policy is not yet a current operator surface
- actor and accountability model are still immature
- branch and PR semantics should not be decided before the Dev module and side-effect rules are real

## Open design questions

- Should `direct-main` be allowed only for docs or other non-trust-critical surfaces?
- Should `pull-request` be mandatory for core trust surfaces?
- Is the mode chosen by project policy, contract, operator profile, or runtime guard?
- Does `gate` approve the write mode before `cut`, or only validate evidence after the fact?
- Should PR creation itself be an adapter receipt rather than a core primitive?
- How should `main` vs active trunk be represented for repos that do not use `main`?

## Anti-promises

Do not claim today:

- Punk can open PRs
- Punk can write directly to `main`
- Punk has a stable git side-effect policy
- Punk has a public `--write-mode` flag
- Punk has selected PR-first or direct-main as the future default

## Promotion trigger

Revisit when agent-driven repo writes become real scope, likely around future Dev module or adapter side-effect work.

Promotion should require:

- R2 research on agent repo-write patterns and failure modes
- an explicit ADR for git side-effect policy
- conformance evals for receipts, review requirements, rollback, and trust-surface protections
- roadmap and status updates before any operator-facing claim
