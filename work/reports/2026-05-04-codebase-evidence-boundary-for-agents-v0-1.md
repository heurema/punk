---
id: report_2026_05_04_codebase_evidence_boundary_for_agents_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-04
updated_at: 2026-05-04
goal_ref: work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md
---

# Codebase Evidence Boundary for Agents v0.1

## Summary

Captured Codebase Evidence Boundary for Agents as R2 advisory research and
eval/spec direction.

Verdict: docs/eval boundary recorded, no implementation selected.

The patch frames future repository assistance for agents as:

```text
bounded local repository evidence
  -> retrieval receipt
  -> clause-indexed Contract Context Pack
  -> advisory assessment
  -> gate/proof path later
```

## What changed

- Added `knowledge/research/2026-05-03-codebase-understanding-for-agents.md`.
- Added `evals/specs/repo-search-adapter-boundary.v0.1.md`.
- Added `evals/specs/codebase-context-pack-boundary.v0.1.md`.
- Updated `docs/adapters/repo-search.md` with Codebase Evidence Boundary
  terminology, mandatory future lexical modes, optional symbol modes, receipt
  fields, warnings, and no-network/side-effect assertions.
- Recorded this side-track work in `work/STATUS.md`, this goal, and this
  report without changing the selected next brownfield verification goal.

## Boundary preserved

Repo-search remains:

```text
Status: parked
Authority: advisory
Current surface: documentation only
```

Retrieval remains advisory evidence only.

The patch adds no:

- CLI behavior;
- runtime storage;
- `.punk/events`;
- `.punk/runs`;
- `.punk/evals`;
- `.punk/contracts`;
- `.punk/decisions`;
- `.punk/proofs`;
- repo scanning implementation;
- file walking implementation;
- MCP integration;
- provider adapters;
- agent execution;
- Dev module runtime;
- vector DB;
- embedding index;
- code graph writer;
- Knowledge Vault runtime;
- Context Compiler runtime;
- new proofpack writer behavior;
- gate decision writer behavior.

## Drift observed

No active-surface drift requiring correction was observed in touched files.

Existing repo-wide wording still contains older negated anti-pattern phrases
such as vector DB or LLM-summary truth warnings. Those are not new active
claims and were not changed in this bounded patch.

## Checks run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `loci skill search punk-workflow`
- `rg -n "active repo-search|active adapter|agent execution is active|MCP integration is active|vector.*truth|code graph.*truth|LLM summary.*truth|understands the codebase" README.md docs knowledge evals || true`
- `rg -n "Codebase Evidence Boundary|repo-search-adapter-boundary|codebase-context-pack-boundary|2026-05-03-codebase-understanding-for-agents" docs knowledge evals`
- `scripts/check.sh docs-governance --files docs/adapters/repo-search.md knowledge/research/2026-05-03-codebase-understanding-for-agents.md evals/specs/repo-search-adapter-boundary.v0.1.md evals/specs/codebase-context-pack-boundary.v0.1.md work/STATUS.md work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md --report work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md`
- `git diff --check`

## Checks not run

- `cargo check --workspace`: not run because this patch changes only docs,
  eval specs, research, and work-ledger artifacts.
- `cargo test --workspace`: not run for the same reason.
- `punk eval run smoke`: not run because no Rust code, CLI behavior, runtime
  storage, broader runtime proofpack writer orchestration, `.punk/proofs`
  writer behavior, acceptance proof behavior, or active eval implementation
  changed.

## Knowledge impact

- Canonical artifacts changed:
  - `docs/adapters/repo-search.md`
  - `work/STATUS.md`
  - `work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md`
  - `work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md`
- Advisory artifacts changed:
  - `knowledge/research/2026-05-03-codebase-understanding-for-agents.md`
  - `evals/specs/repo-search-adapter-boundary.v0.1.md`
  - `evals/specs/codebase-context-pack-boundary.v0.1.md`
- Project-memory claims affected:
  - Future codebase evidence for agents must be local-first, bounded,
    receipt-bearing, clause-indexed, and advisory unless later promoted.
- Docs / ADRs / evals possibly stale:
  - Existing repo-search research may need supersession or accepted-status
    review if this note is later promoted.
- Active / parked / future scope affected:
  - Repo-search remains parked.
  - Contract Context Pack remains advisory evidence.
  - Dev module, MCP, vector, graph, remote search, and agent execution remain
    future or parked.
- Public narrative impact:
  - None.
- Derived views to rebuild later:
  - None active.
- Follow-up goals or drift findings:
  - Optional ADR only if this research is promoted.
  - Future fixture-backed eval implementation before adapter promotion.
- Unknowns / contradictions:
  - Exact external source details were not fully revalidated in this patch and
    remain directional evidence only.
- Out of scope:
  - Runtime, CLI, MCP, agent execution, vector DB, graph runtime, repo-search
    runtime, Dev module, gate writer, broader runtime proofpack writer
    orchestration, `.punk/proofs` writer behavior, acceptance proof behavior.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  canonical_docs:
    - docs/adapters/repo-search.md
    - work/STATUS.md
  eval_specs:
    - evals/specs/repo-search-adapter-boundary.v0.1.md
    - evals/specs/codebase-context-pack-boundary.v0.1.md
  work_artifacts:
    - work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md
    - work/reports/2026-05-04-codebase-evidence-boundary-for-agents-v0-1.md
  research:
    - knowledge/research/2026-05-03-codebase-understanding-for-agents.md
  reason: "Records R2 advisory Codebase Evidence Boundary research/eval direction while preserving repo-search as parked, advisory, documentation-only and without activating runtime, CLI, MCP, agent execution, vector DB, code graph runtime, gate writer, broader runtime proofpack writer orchestration, `.punk/proofs` writer behavior, acceptance proof behavior, or Dev module behavior."
```
