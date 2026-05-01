---
id: report_2026_05_01_greenfield_init_docs_flow_inspect_polish_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md
---

# Greenfield Init Docs and Flow Inspect Polish v0.1

## Summary

- Clarified that `punk init <project-id>` initializes the current directory in place.
- Clarified that init does not create a new subdirectory named `<project-id>`.
- Kept active init wording greenfield-only and compact `.punk/memory/` only.
- Removed the repo-internal `preview_goal_ref` from `punk flow inspect` human preview output.
- Checked `.punk/project` and `.gitignore` consistency; no `.gitignore` change was needed.

## Files Changed

- `crates/punk-cli/src/main.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md`
- `work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md`

## Current-Directory Init Wording

- CLI usage now says init writes into the current directory in place.
- CLI usage now says init does not create a new subdirectory named `<project-id>`.
- README and START-HERE carry the same user-facing wording.
- CRATE-STATUS records the same current behavior for the active CLI surface.

## Greenfield Compact Layout Boundary

The wording keeps `punk init <project-id>` scoped to:

- greenfield-only;
- compact `.punk/memory/` tracked durable memory;
- `.punk/README.md` and `.punk/project.toml` marker/setup files;
- no root-level user-project `work/`, `knowledge/`, `docs/adr/`, or `publishing/`;
- no `.punk/runtime` or derived runtime stores.

## Flow Inspect Preview

`punk flow inspect` no longer emits:

```text
preview_goal_ref: work/goals/...
```

The preview still shows local flow/event kernel evidence, guard code, event kind/status, and preview flow id, but it no longer exposes a repo-internal goal ref as if it were user project state.

## .punk/project and .gitignore Consistency

Checked refs for `.punk/project/`, `.punk/project`, and `.punk/project.toml`.

Findings:

- Active marker/setup file is `.punk/project.toml`.
- `.gitignore` does not ignore `.punk/project.toml`.
- `.gitignore` ignores only future runtime/derived `.punk` directories such as `.punk/runtime/`, `.punk/events/`, `.punk/runs/`, `.punk/evals/`, `.punk/contracts/`, `.punk/decisions/`, `.punk/proofs/`, `.punk/indexes/`, and `.punk/views/`.
- No active `.punk/project/` path mismatch was found.

No `.gitignore` change was needed.

## Boundaries Preserved

- No brownfield behavior was added.
- No grayfield behavior was added.
- No contracts, gate/proof runtime, Writer, Conformance Pack runtime, replayability runtime, or spec-as-source behavior was added.
- No new runtime `.punk` storage or new CLI surface was added.

## Checks Run

- `cargo test -p punk-cli -- --nocapture` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `~/.local/bin/punk-dev flow inspect` output check - PASS; no `preview_goal_ref` or `work/goals/` ref was emitted.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-cli/src/main.rs README.md docs/product/START-HERE.md docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md --report work/reports/2026-05-01-greenfield-init-docs-flow-inspect-polish-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next Selected Goal

`work/goals/goal_pause_after_greenfield_init_checkpoint_v0_1.md`

Reason: the greenfield init operational hardening and docs/UX polish are complete; the safe next state is a pause/status checkpoint before selecting any broader product work.

## Doc impact
```yaml
  classification: code-doc
  reason: "Clarifies active init CLI/docs behavior and removes a misleading repo-internal flow inspect preview ref without expanding product scope."
```
