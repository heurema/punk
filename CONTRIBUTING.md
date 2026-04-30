# Contributing to punk

Thanks for your interest in contributing.

`punk` is an **experimental**, early-stage, local-first bounded work kernel.
Contributions should preserve the repo's current build posture:
- active-core first;
- clear source-of-truth boundaries;
- no silent promotion of parked capabilities;
- one bounded diff at a time.

## Read this first

Before making a non-trivial change, read:

1. `README.md`
2. `AGENTS.md`
3. `work/STATUS.md`
4. `docs/product/START-HERE.md`
5. `docs/product/PUNK-LAWS.md`
6. `docs/product/DOCUMENTATION-MAP.md`
7. `docs/product/RESEARCH-GATE.md`

Then read the exact files you plan to change.

## Repo workflow rules

This repository uses a manual work ledger.

Before meaningful repo work:

1. Run Research Gate preflight and classify the task as `R0`, `R1`, `R2`, or `R3`.
2. Check `work/STATUS.md`.
3. Follow `selected_next` unless the maintainer or user explicitly changes scope.
4. Keep one bounded diff per task.
5. Update the relevant goal, report, and `work/STATUS.md` for meaningful changes.

If you intentionally work outside `selected_next`, say so explicitly in the PR.

## What is useful right now

At the current stage, the most useful contributions are:

- bounded fixes to active-core behavior;
- docs and work-ledger consistency improvements;
- deterministic checks in `scripts/`;
- careful crate-level progress that matches the documented active target;
- repo hygiene and contributor-experience improvements that do not widen product scope.

## Ground rules

### 1) Do not promote parked capabilities silently

Do not present future ideas as current operator surfaces.

Examples of not-active-yet surfaces include:
- autonomous coding agent execution;
- provider adapters;
- MCP integration;
- marketplace surfaces;
- cloud sync / SaaS control plane.

If a change promotes a future surface, it must be explicitly documented and justified.

### 2) Keep source-of-truth boundaries intact

Respect:
- `work/STATUS.md` as the live work-state source of truth;
- `docs/product/*` as canonical product/docs truth;
- `knowledge/` as research/ideas, not product truth;
- `publishing/` as public-build material, not operator truth.

### 3) Run the required checks

Always run:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
```

When product/process docs, reports, or work-ledger artifacts change, also run:

```bash
scripts/check.sh docs-governance --files <changed-files> --report <report-path>
```

When Rust code changes, also run:

```bash
cargo check --workspace
```

### 4) Prefer bounded, reviewable diffs

Do not mix unrelated cleanup into the same PR.

Good examples:
- one work-ledger/report/doc governance change;
- one bounded core crate change;
- one repo-governance/community-health change.

### 5) Keep repo-tracked artifacts clean

In repo-tracked artifacts:
- use repo-relative paths only;
- do not write `.punk/` runtime state unless explicitly scoped;
- do not leave absolute local paths behind in committed files.


## PR Intake Gate

This repository uses a deterministic PR Intake Gate before ordinary code review.

The gate is intentionally conservative for external contributors and intentionally low-friction for trusted repository authors:

- it runs from trusted base-branch code through `pull_request_target`;
- it reads PR metadata, changed-file metadata, and author repository permission through the GitHub API;
- it does not checkout, import, install, or execute PR head code;
- trusted authors with `admin`, `maintain`, or `write` repository permission pass intake automatically;
- if permission lookup is unavailable, `OWNER`, `MEMBER`, and `COLLABORATOR` author associations pass as a fallback;
- external PRs are labeled with `intake/pass`, `intake/needs-linked-intent`, `intake/needs-more-context`, `intake/no-code-alternative`, or `intake/high-risk`;
- first-time external contributors also receive `intake/first-time-contributor`;
- label and bot-comment updates are best-effort visibility aids; the check verdict comes from the deterministic PR metadata evaluation;
- maintainers can accept a non-high-risk external PR with `intake/accepted-for-pr`;
- maintainers can bypass intake with `maintainer/override-intake` when they explicitly accept responsibility.

Direct external PRs are intended only for small low-risk edits. Non-trivial external PRs should link an Issue, Discussion, or repo-tracked intent artifact such as `work/goals/...`, `work/reports/...`, `docs/adr/...`, `knowledge/research/...`, or `evals/specs/...`, and fill the external contributor context sections in the PR template.

External PRs touching high-risk surfaces such as `.github/**`, `scripts/**`, `crates/**`, `docs/product/**`, `work/**`, `knowledge/**`, `evals/**`, public narrative, site, or brand assets require maintainer attention before ordinary code review.

Shared gate engine:

- runtime action: `heurema/repo-governance/actions/pr-intake-gate@v0.1.0` in `.github/workflows/pr-intake-gate.yml`;
- local policy: `.github/pr-intake-gate.yml`;
- reusable engine tests live in the shared `heurema/repo-governance` repository.

For policy changes, open a PR and verify the `pr-intake-gate` check.

## Commit sign-off (DCO)

This repository uses the **Developer Certificate of Origin (DCO)** instead of a CLA.

Sign off every commit:

```bash
git commit -s -m "Your message"
```

That adds:

```text
Signed-off-by: Your Name <you@example.com>
```

By doing so, you certify the terms in `DCO.md`.

## Pull request expectations

Please use the PR template and include:
- Research Gate classification;
- whether the diff follows `selected_next` or is an explicit scope override;
- clear in-scope / out-of-scope boundaries;
- DocImpact / work-ledger impact;
- commands run and evidence.

## Security

Do **not** report vulnerabilities with full details in a public issue.
See `SECURITY.md` for reporting guidance.

## Code of conduct

By participating in this project, you agree to follow `CODE_OF_CONDUCT.md`.
