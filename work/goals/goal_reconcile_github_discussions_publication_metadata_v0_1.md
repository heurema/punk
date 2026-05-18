---
id: goal_reconcile_github_discussions_publication_metadata_v0_1
title: "Reconcile GitHub Discussions publication metadata v0.1"
status: done
owner: "vitaly"
authority: canonical
module: "public-narrative"
priority: P2
created_at: 2026-05-18
updated_at: 2026-05-18
selected_at: 2026-05-18
started_at: 2026-05-18
completed_at: 2026-05-18
blocked_by: []
scope:
  include:
    - "publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md"
    - "publishing/channels/github-discussions-community-lab.md"
    - "work/goals/goal_reconcile_github_discussions_publication_metadata_v0_1.md"
    - "work/reports/2026-05-18-github-discussions-publication-metadata-reconciliation-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - "code"
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "evals/**"
    - "publishing/publications/**"
    - "publishing/metrics/**"
    - "GitHub API calls"
    - "actual publication"
    - "new publication receipt"
    - "metrics snapshot"
    - "Community Lab digest"
    - "bot/adapters/integrations"
    - "raw transcript storage"
    - "automatic issue/goal creation"
    - "PubPunk runtime"
    - "Module Host behavior"
acceptance:
  - "Links the GitHub Discussions Start Here post metadata to the existing manual publication receipt."
  - "Updates the GitHub Discussions Community Lab channel descriptor to reflect the existing manual launch URL."
  - "Keeps the existing publication receipt as the publication evidence."
  - "Keeps GitHub Discussions replies advisory/raw and not project truth."
  - "Adds no product/runtime/CLI/bot/adapter behavior."
  - "Adds no external publication, GitHub API call, new receipt, metrics snapshot, digest, or module-host behavior."
  - "Leaves selected_next unchanged."
knowledge_refs:
  - "publishing/publications/2026-05-13-community-lab-cycle-0-github-discussions.md"
  - "work/reports/2026-05-13-community-lab-cycle-0-publication-receipt.md"
  - "publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md"
  - "publishing/channels/github-discussions-community-lab.md"
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "docs/modules/pubpunk.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-18-github-discussions-publication-metadata-reconciliation-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This reconciles public-narrative metadata and publishing boundary evidence using existing repo-tracked publication receipts and module-boundary docs. No external research is needed because it does not verify or publish to external platforms."
  research_refs:
    - "publishing/publications/2026-05-13-community-lab-cycle-0-github-discussions.md"
    - "work/reports/2026-05-13-community-lab-cycle-0-publication-receipt.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: public-claim
  required_updates:
    - "publishing/posts/**"
    - "publishing/channels/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The change reconciles repo-tracked post/channel metadata with an existing manual publication receipt without publishing, adding automation, or changing product/runtime behavior."
---

# Reconcile GitHub Discussions publication metadata v0.1

## Context

The manual publication receipt for Community Lab Cycle 0 already exists:

`publishing/publications/2026-05-13-community-lab-cycle-0-github-discussions.md`

The linked post and channel metadata still described the surface as not yet
published. That created a false receipt gap for future PubPunk inventory.

## Intent

Reconcile the metadata around the existing receipt without creating new
publishing behavior.

## In scope

- Link the post metadata to the existing receipt.
- Mark the GitHub Discussions channel descriptor as active/manual.
- Record the existing external URL from the receipt.
- Add work-ledger evidence for the reconciliation.

## Out of scope

- Publishing externally.
- Creating a new publication receipt.
- Creating a metrics snapshot.
- Preparing a Community Lab digest.
- Adding runtime publishing, PubPunk runtime, adapters, bots, GitHub API calls,
  CLI behavior, or Module Host behavior.

## Outcome

Done in
`work/reports/2026-05-18-github-discussions-publication-metadata-reconciliation-v0-1.md`.

The patch updates metadata only. The existing receipt remains the publication
evidence, and GitHub Discussions remains a Community Visor, not project truth.
