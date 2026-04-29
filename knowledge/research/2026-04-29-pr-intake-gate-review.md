---
kind: research-note
status: active
authority: advisory
created: 2026-04-29
related_goal: work/goals/goal_add_pr_intake_gate_v0_1.md
related_report: work/reports/2026-04-29-pr-intake-gate-v0-1.md
---

# PR Intake Gate Review

## Question

Should Punk adopt Signum-style PR intake before ordinary code review, and if yes, how should it fit Punk's current repo-governance and trust posture?

## Decision context

Punk is public and early-stage. The repo already requires Research Gate preflight, Work Ledger alignment, bounded diffs, DCO sign-off, and proof-oriented validation. The missing mechanism is an automated first-pass PR intake check that prevents non-trivial or high-risk PRs from entering ordinary code review without linked intent or maintainer responsibility.

This touches GitHub workflow behavior and repository governance. It does not change Punk runtime behavior, CLI behavior, crates, `.punk/` storage, gate decisions, or proofpack semantics.

## Sources reviewed

Checked on: 2026-04-29.

| Source | Tier | Why used | Punk takeaway |
|---|---|---|---|
| Signum `.github/workflows/pr-intake-gate.yml` | B | Direct prior-art workflow from sibling project. | Run intake on `pull_request_target`, but checkout trusted base code only. |
| Signum `.github/pr-intake-gate.yml` | B | Direct prior-art policy config. | Keep labels, trivial thresholds, linked intent patterns, and high-risk paths config-driven. |
| Signum `scripts/pr_intake_gate.py` | B | Direct prior-art implementation. | Use Python stdlib, GitHub REST metadata, dry-run fixture testing, label sync, and bot-comment upsert. |
| Signum `CONTRIBUTING.md` and PR template | B | Contributor-facing policy. | Explain that non-trivial work needs linked intent and risky paths need maintainer override. |
| GitHub Actions docs for `pull_request_target` | A | Official event semantics. | `pull_request_target` runs in base context; therefore PR head code must not be checked out or executed by the intake job. |
| GitHub Actions security hardening docs | A | Official security guidance. | Treat PR content as untrusted input; avoid shell interpolation and untrusted code execution. |
| Punk `START-HERE`, `PUNK-LAWS`, `RESEARCH-GATE`, `CONTRIBUTING.md` | Canonical internal | Source of truth for current build posture and repo workflow. | The gate must preserve Work Ledger, Research Gate, explicit overrides, and no hidden authority. |

Source refs:

- `https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#pull_request_target`
- `https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/RESEARCH-GATE.md`
- `CONTRIBUTING.md`

## Existing systems / prior art

Signum uses a deterministic PR Intake Gate before ordinary code review:

- workflow runs on `pull_request_target`;
- trusted base code is checked out;
- changed files are fetched through GitHub REST API;
- PR head code is not executed;
- labels classify the PR as pass, needs issue, high risk, or maintainer override;
- local fixture tests cover helper and verdict behavior.

Punk already has a stronger repo-specific governance model than Signum:

- Research Gate classification;
- Work Ledger selected-next discipline;
- repo-tracked goals, reports, ADRs, research notes, and eval specs;
- strict high-risk truth surfaces such as `docs/product/**`, `work/**`, `knowledge/**`, and `evals/**`.

## Failure modes found

1. **Unsafe `pull_request_target` usage**: checking out or executing PR head code would expose write token context to untrusted code.
2. **Intent-free non-trivial PRs**: large changes can consume review time without a linked problem, goal, or decision artifact.
3. **High-risk surface drift**: changes to workflows, scripts, crates, canonical product docs, work ledger, research, evals, public narrative, site, or brand assets need maintainer attention before line-level review.
4. **Label/comment noise**: stale intake labels and duplicate bot comments make PR state hard to read.
5. **External issue-only mismatch**: Punk often tracks intent in repo artifacts, not only GitHub Issues or Discussions.
6. **Branch protection blind spot**: tracked files can add the workflow, but GitHub branch protection must still require it outside the repo.

## Options considered

### Option A: Copy Signum gate exactly

Pros: proven sibling-project pattern, simple.

Cons: intent model is too GitHub-issue-centric for Punk; high-risk paths do not reflect Punk's source-of-truth surfaces.

### Option B: Add only docs, no workflow

Pros: lowest risk.

Cons: does not create enforceable intake signal; relies on reviewers to notice missing intent manually.

### Option C: Adopt a Punk-specific deterministic gate

Pros: keeps the safe Signum execution model, adds Punk-native linked intent, and treats high-risk truth surfaces conservatively.

Cons: requires branch protection configuration outside tracked files to become a required merge gate.

## Recommendation

Adopt Option C.

Add a deterministic, stdlib-only PR intake gate that:

- runs through `pull_request_target` against trusted base code;
- reads only PR metadata and changed-file metadata through GitHub API;
- never checks out, imports, installs, or executes PR head code;
- auto-creates configured intake labels when missing, then labels `intake/pass`, `intake/needs-linked-intent`, or `intake/high-risk`;
- allows `maintainer/override-intake` as an explicit ledgered human responsibility signal;
- accepts Punk-native linked intent refs in addition to GitHub Issues and Discussions;
- treats Punk source-of-truth and trust surfaces as high risk;
- includes local dry-run fixture tests.

## Adoption map: adopt / defer / park / avoid

| Classification | Items |
|---|---|
| adopt | Trusted-base `pull_request_target` intake workflow; config-driven labels/path policy with trusted label definitions; stdlib Python API-only implementation; Punk-native linked intent refs; dry-run fixture tests; contributor docs. |
| defer | Repository settings/branch protection changes that require GitHub admin configuration outside tracked files; DCO status check automation. |
| park | Any remote policy service, SaaS intake bot, external AI reviewer requirement, or hidden contributor scoring. |
| avoid | Checking out or executing PR head code in `pull_request_target`; parsing rendered GitHub UI; allowing models or bots to write final acceptance decisions. |

## What stays out of scope

- No Rust code changes.
- No active CLI changes.
- No `.punk/` runtime state.
- No gate decision, proofpack, or product-runtime acceptance behavior.
- No external AI reviewer requirement.
- No GitHub branch-protection API mutation from this diff.

## Impact on roadmap

This is repo governance, not product capability. It supports active-core trust work by reducing unbounded PR intake, but it does not promote any parked product surface.

## Required evals

No product eval is required because runtime behavior is unchanged.

Required deterministic tests:

- helper path matching and bot-comment recognition;
- trivial low-risk PR passes;
- non-trivial PR without linked intent fails;
- Punk-native linked intent passes;
- product docs and workflows are high risk;
- maintainer override passes high-risk changes.

## Required docs/ADRs/contracts

- Work goal and report.
- Research note.
- Contributor docs and PR template update.
- Changelog note.

No ADR required because this does not change core architecture, module interfaces, storage, public CLI, gate/proof boundaries, or active product behavior.

## Open questions

1. Should GitHub branch protection require `pr-intake-gate` before merge?
2. Should DCO sign-off become a separate required check later?
3. Should small changes to `README.md` remain low-risk, or should README be promoted into high-risk once public positioning stabilizes?

## Follow-up: trusted author split

Checked on: 2026-04-29.

After the initial gate landed, Punk adopted a maintainer-author fast path so the required gate does not slow down trusted repository work while still protecting external intake.

The chosen mechanism is:

1. Query GitHub repository permission for the PR author.
2. Treat `admin`, `maintain`, and `write` as trusted.
3. If permission lookup is unavailable or inconclusive, fallback to `author_association` and trust `OWNER`, `MEMBER`, and `COLLABORATOR`.
4. Apply strict external intake only when the author is not trusted.

Why this fits Punk:

- it keeps the required status check uniform for every PR;
- it avoids hidden UI parsing;
- it keeps maintainer work low-friction;
- it uses structured GitHub metadata before fallback heuristics;
- it keeps external high-risk and non-trivial PRs bounded by linked intent and explicit context.

Additional adoption map:

| Classification | Items |
|---|---|
| adopt | Permission-first trusted author detection; `author_association` fallback; external-only context checks; first-time external contributor advisory label; maintainer `intake/accepted-for-pr` label for non-high-risk external intent acceptance. |
| defer | Organization/team-specific allowlists and DCO automation. |
| park | Contributor reputation scoring or model-based trust decisions. |
| avoid | Applying heavyweight external contributor intake to maintainers by default. |

## Follow-up: label/comment writes as side effects

Checked on: 2026-04-29.

Opening the trusted-author follow-up PR showed that GitHub token write operations can fail independently of the deterministic intake verdict:

```text
GitHub API POST /repos/heurema/punk/issues/4/labels failed: HTTP 403: Resource not accessible by integration
```

Forward policy:

- request both `issues: write` and `pull-requests: write` for the intake workflow;
- treat label and bot-comment writes as best-effort visibility side effects;
- keep the required status-check verdict based on PR metadata, changed-file metadata, author trust, linked intent, and configured policy;
- do not let a label/comment write denial turn a valid pass/fail verdict into an infrastructure error.
