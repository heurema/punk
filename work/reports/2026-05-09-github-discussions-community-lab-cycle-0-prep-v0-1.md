---
id: report_2026_05_09_github_discussions_community_lab_cycle_0_prep_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-09
updated_at: 2026-05-09
goal_ref: work/goals/goal_prepare_github_discussions_community_lab_cycle_0_v0_1.md
---

# GitHub Discussions Community Lab Cycle 0 Prep v0.1

## Summary

Prepared the concrete GitHub Discussions launch draft for Punk Community Lab
cycle 0.

The patch creates a manual runbook, ready-to-copy Start Here discussion draft,
and planned/manual channel descriptor. It keeps GitHub Discussions as a
Community Visor, not project truth.

This is not an implementation or publication patch.

## Files changed

- `knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md`
- `knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md`
- `publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md`
- `publishing/channels/github-discussions-community-lab.md`
- `work/goals/goal_prepare_github_discussions_community_lab_cycle_0_v0_1.md`
- `work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md`
- `work/STATUS.md`

## Validation run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md publishing/channels/github-discussions-community-lab.md work/goals/goal_prepare_github_discussions_community_lab_cycle_0_v0_1.md work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md work/STATUS.md --report work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md` - PASS.
- `git diff --check` - PASS.
- `cargo check --workspace` - not run; no Rust code changed.

## Doc impact

```yaml
doc_impact:
  classification: public-claim
  reason: "Adds advisory GitHub Discussions Community Lab launch materials without publication, GitHub API calls, active product behavior, runtime behavior, CLI behavior, bot/adapters, CommunityPunk, Topic Graph, DAO, token, funding, or governance mechanics."
  canonical_docs: []
  ops:
    - knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md
    - knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md
  public_drafts:
    - publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md
  channels:
    - publishing/channels/github-discussions-community-lab.md
  work_artifacts:
    - work/goals/goal_prepare_github_discussions_community_lab_cycle_0_v0_1.md
    - work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: None.
- Active runtime scope unchanged.
- Public narrative draft/channel descriptor added.
- No publication receipt created because no publication happened.
- Community Lab remains advisory/manual.
- GitHub Discussions is a visor, not truth.
- Automated responder remains deferred/external.
- CommunityPunk runtime remains parked/future.
- DAO/token/funding remain parked/avoided.
- Next human action: manually enable/open GitHub Discussions, post draft, then
  create publication receipt.

## Drift observed

- None requiring product-doc changes in this patch.
- The surface-agnostic launch pack already allowed GitHub Discussions as an
  option. This patch narrows cycle 0 prep to the maintainer-selected GitHub
  Discussions surface without making that surface canonical truth.
- Publication receipt remains intentionally absent until manual publication.

## Out of scope

No product behavior, runtime behavior, CLI behavior, Rust code, `.punk/`
runtime writes, GitHub API call, GitHub Issue, GitHub Discussions settings
automation, bot, live adapter, live send, raw chat storage, raw transcript
archive, automatic issue/goal creation, Topic Graph implementation,
CommunityPunk runtime, module-host behavior, docs/product promotion, ADR, eval
spec, publication receipt, actual publication, token, DAO, microgrant, bounty,
treasury, or funding mechanism was added.
