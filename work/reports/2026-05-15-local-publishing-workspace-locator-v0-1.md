---
id: report_2026_05_15_local_publishing_workspace_locator_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-15
updated_at: 2026-05-15
goal_ref: work/goals/goal_add_local_publishing_workspace_locator_v0_1.md
---

# Local Publishing Workspace Locator v0.1

## Summary

Added the first active local publishing prerequisite: `punk publishing locate`.

The command resolves a publishing workspace from `.punk/publishing.toml` plus a
local-only `.punk/publishing.local.toml` pointer and reports located/blocked
status in human or JSON form.

This is not publishing automation.

## Runtime slice selected

- Selected: local-only publishing workspace resolver.
- Reason: PubPunk-style workflows need to find the durable local authoring
  workspace before any module can plan, draft, publish, or record receipts.
- Active scope: read-only binding validation and local workspace location.
- Not selected: publishing execution, drafting, browser/API calls, credential
  access, adapters, bots, automatic receipts, GitHub behavior, issue/PR
  creation, gate decisions, proofpacks, or acceptance claims.

## Files changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `crates/punk-eval/src/lib.rs`
- `scripts/check_docs_governance.py`
- `evals/cases/docs/active-cli-surface/README.md`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PUBLIC-NARRATIVE.md`
- `work/STATUS.md`
- `work/goals/goal_add_local_publishing_workspace_locator_v0_1.md`
- `work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md`

## Implementation notes

- Added `locate_publishing_workspace` in `punk-project`.
- Added `PublishingLocateReport` human and JSON renderers.
- Added `punk publishing locate [--project-root <path>] [--json]`.
- Added fail-closed blockers for missing binding, unreadable config, invalid
  schema, mismatched workspace refs, unsafe root shape, and unavailable
  workspace roots.
- Rejected committed top-level `workspace_root` in `.punk/publishing.toml`.
- Kept host workspace roots in local-only `.punk/publishing.local.toml`.
- Added smoke coverage proving the resolver does not write files or invoke
  publishing, browser, network API, credential, adapter, or bot behavior.
- Updated docs-governance active CLI truth so `punk publishing locate` is
  checked as implemented rather than flagged as overclaim.

## Validation run

Validation was run after implementation. Results:

- `cargo test -p punk-project`: PASS
- `cargo test -p punk-cli`: PASS
- `cargo test -p punk-eval`: PASS
- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `cargo run -q -p punk-cli -- publishing locate --project-root <external Goalrail checkout> --json`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files README.md crates/punk-cli/src/main.rs crates/punk-eval/src/lib.rs crates/punk-project/src/lib.rs docs/product/CRATE-STATUS.md docs/product/PUBLIC-NARRATIVE.md docs/product/ROADMAP.md docs/product/START-HERE.md evals/cases/docs/active-cli-surface/README.md scripts/check_docs_governance.py work/STATUS.md work/goals/goal_add_local_publishing_workspace_locator_v0_1.md work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md --report work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md`: PASS with 5 unchanged `docs/product/PUBLIC-NARRATIVE.md` glossary/definition-shape warnings.
- `git diff --check`: PASS

The external-checkout verification path is intentionally omitted from this
repo-tracked report.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The patch adds a bounded public CLI command for local publishing workspace location."
  touched_surfaces:
    - crates/punk-project/src/lib.rs
    - crates/punk-cli/src/main.rs
    - crates/punk-eval/src/lib.rs
    - scripts/check_docs_governance.py
    - evals/cases/docs/active-cli-surface/README.md
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PUBLIC-NARRATIVE.md
    - work/STATUS.md
    - work/goals/goal_add_local_publishing_workspace_locator_v0_1.md
    - work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PUBLIC-NARRATIVE.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `README.md`,
  `docs/product/START-HERE.md`, `docs/product/ROADMAP.md`,
  `docs/product/CRATE-STATUS.md`, and
  `docs/product/PUBLIC-NARRATIVE.md`.
- Active CLI surface now includes `punk publishing locate`.
- Active runtime scope unchanged.
- Publishing execution remains inactive.
- PubPunk remains parked/future.
- Repo-tracked `publishing/` remains public narrative truth.
- `.punk/publishing.local.toml` is local-only pointer state, not project truth.
- External platforms remain visors/surfaces, not sources of truth.
- No publication receipt was created.
- No bot, adapter, browser/API, credential, issue/PR, DAO, token, or funding
  behavior was added.

## Drift observed

- Goalrail already had a local publishing binding shape, but installed Punk did
  not yet have the referenced `punk publishing locate` command.
- The product path had drifted toward manual publication/digest operations; the
  maintainer corrected direction toward zero-manual Punk. This slice starts at
  local resolution only, not external action.

## Out of scope

- PubPunk runtime
- Publishing execution
- Draft generation
- Browser automation
- GitHub, Telegram, Discord, LinkedIn, or other platform APIs
- Provider adapters
- Bots
- Credential or token reads
- Automatic publication receipts
- Automatic issue or PR creation
- External surfaces as truth
- Raw transcript storage
- Gate decision writer
- Proofpack writer expansion
- Acceptance claim writer
- DAO, token, or funding behavior

## Next code slice

Add a local publishing plan/draft inventory reader that lists candidate posts
and required receipt gaps from the located workspace without publishing,
calling APIs, reading credentials, or creating receipts.
