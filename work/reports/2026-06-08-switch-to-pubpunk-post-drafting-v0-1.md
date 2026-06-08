---
id: report_2026_06_08_switch_to_pubpunk_post_drafting_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: null
selected_next: work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
related_goals:
  - work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md
  - work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
related_docs:
  - docs/modules/codebase-study.md
  - docs/modules/pubpunk.md
  - docs/product/MODULES.md
doc_impact:
  classification: work-ledger-only
  refs:
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
    - work/reports/2026-06-08-switch-to-pubpunk-post-drafting-v0-1.md
research_gate:
  classification: R1
  required: true
---

# Switch selected_next to PubPunk post drafting v0.1

## Summary

Switched the active Work Ledger selection from Codebase Study to PubPunk after
the Codebase Study capability/privacy checkpoint landed on `main`.

Selected next:

```text
work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
```

## Rationale

Codebase Study is at a clean pause point:

- module boundary is defined;
- conformance packet is applied;
- source observation request packet is defined;
- capability/privacy boundary is defined and merged;
- active scanner, reader, content read, source hashing, storage, lab execution,
  gate/proof, and acceptance behavior remain inactive.

The remaining Codebase Study receipt/evidence boundary is still a valid future
checkpoint, but it does not block the user-requested PubPunk work.

PubPunk is now more urgent because it is needed for writing posts across
different resources. The next safe PubPunk slice is a request packet for post
drafting, not active drafting or publishing.

## Decision

Select `goal_add_pubpunk_post_draft_request_packet_v0_1` as the next bounded
checkpoint.

The goal should add the first side-effect-free packet for explicit content
intent, target resource/channel refs, voice/style refs, constraints, and
expected draft outputs.

## Deferred

Deferred until later explicit goals:

- draft text generation;
- provider/model calls;
- channel-specific external research;
- filesystem reads;
- publishing workspace scans;
- PubPunk module execution;
- public CLI behavior;
- adapter invocation;
- external publishing;
- metrics collection;
- receipt writing;
- event writing;
- gate/proof behavior;
- acceptance claims.

## Doc impact

```yaml
doc_impact:
  classification: work-ledger-only
  reason: "Records an explicit user scope override from Codebase Study to PubPunk and selects one bounded PubPunk drafting goal."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
    - work/reports/2026-06-08-switch-to-pubpunk-post-drafting-v0-1.md
  required_updates:
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md
    - work/reports/2026-06-08-switch-to-pubpunk-post-drafting-v0-1.md
```

## Validation

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files work/STATUS.md work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md work/reports/2026-06-08-switch-to-pubpunk-post-drafting-v0-1.md` passed with zero warnings.
- `git diff --check` passed.

## Not tested

No Rust tests are required for this work-ledger-only switch. No Rust code,
runtime behavior, CLI behavior, PubPunk module execution, provider call,
adapter invocation, publishing behavior, `.punk` state, gate/proof behavior,
or acceptance claim was changed.
