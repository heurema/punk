---
id: report_2026_04_29_project_knowledge_vault_direction_v0_1
goal_id: goal_record_project_knowledge_vault_direction_v0_1
actor: vitaly
created_at: 2026-04-29
kind: docs-research-direction
---

## Summary

Recorded Project Knowledge Vault direction v0.1 as a docs/research-only patch.

The patch clarifies that Knowledge Vault is not RAG. For Punk, the direction is a repo-first artifact ledger with typed claims/facts/relations, receipts/provenance, code graph and project graph, compiled derived views, context compiler direction, and stale/conflict/unknown handling.

No runtime implementation was added.

## Research Gate

Classification: R2
Required: yes
Decision: Proceed as docs/research-only direction.

Rationale: this records architecture/product direction for Project Memory and Knowledge Vault without implementation.

Research refs used:

- `README.md`
- `AGENTS.md`
- `work/STATUS.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/DOGFOODING.md`
- `docs/product/LINEAGE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/adr/ADR-0016-contract-context-pack-boundary.md`
- `evals/specs/context-pack-boundary.v0.1.md`
- user-provided external research list recorded in `knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md`

## Changed Files

- `knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md`
- `knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `work/STATUS.md`
- `work/goals/goal_record_project_knowledge_vault_direction_v0_1.md`
- `work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md`

## Knowledge impact

- Canonical artifacts changed:
  - `docs/product/PROJECT-MEMORY.md` lightly records Knowledge Vault direction.
  - `docs/product/ARCHITECTURE.md` lightly records future Context Compiler boundary.
  - `docs/product/DOCUMENTATION-MAP.md` registers the new research/idea references as supporting/history refs.
  - `work/STATUS.md` records completion and restores the proofpack writer boundary selected next.
- Direction recorded:
  - Knowledge Vault != RAG.
  - Knowledge Vault direction is repo-first artifact ledger + typed claims/facts/relations + receipts/provenance + code/project graph + compiled derived views + context compiler + drift/staleness/conflict handling.
  - RAG/vector search is late auxiliary candidate discovery only.
- Future work created:
  - Future ADR may promote the direction.
  - Knowledge Impact Report convention should be separate next patch.
  - Full implementation requires later goals/contracts/evals.
- Out of scope:
  - code, Rust, CLI, runtime storage, `.punk`, graph DB, embeddings, vector DB, RAG pipeline, compiled wiki generator, context compiler, daemon, MCP runtime, retrieval adapter, Knowledge Vault implementation, Knowledge Impact Report convention, proofpack writer work.
- Derived/runtime behavior added: none.

## Drift observed

This patch reduces the risk that Project Memory collapses into RAG, hidden agent memory, or convenient compiled wiki pages by recording the canonical-vs-derived split, authority metadata, provenance, stale/conflict surfacing, and context-pack direction.

No new unrelated drift was introduced intentionally. The existing docs-governance warning about `Research notes` remains a known low-severity drift finding in `work/STATUS.md`.

## Scope boundaries preserved

No code/runtime/CLI/storage/adapters/graph DB/embeddings/Knowledge Vault implementation was added.

No `.punk` runtime state, schemas, Rust changes, graph DB, vector DB, RAG pipeline, compiled wiki generator, context compiler, daemon, MCP runtime, retrieval adapter, Knowledge Impact Report convention, or proofpack writer work was added.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Recorded Project Knowledge Vault direction v0.1 as docs/research-only direction."
  touched_surfaces:
    - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
    - knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/ARCHITECTURE.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
    - work/goals/goal_record_project_knowledge_vault_direction_v0_1.md
    - work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md
  required_updates:
    - docs/product/PROJECT-MEMORY.md
    - docs/product/ARCHITECTURE.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - git diff --check
    - python3 scripts/check_research_gate.py
    - python3 scripts/check_work_ledger.py
    - scripts/check.sh docs-governance --files knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md docs/product/PROJECT-MEMORY.md docs/product/ARCHITECTURE.md docs/product/DOCUMENTATION-MAP.md work/STATUS.md work/goals/goal_record_project_knowledge_vault_direction_v0_1.md work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md --report work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md
    - cargo check --workspace
    - cargo test --workspace
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md docs/product/PROJECT-MEMORY.md docs/product/ARCHITECTURE.md docs/product/DOCUMENTATION-MAP.md work/STATUS.md work/goals/goal_record_project_knowledge_vault_direction_v0_1.md work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md --report work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md` - PASS with 0 failures and 6 warnings
  - Warnings: existing duplicate-definition candidates in `docs/product/ARCHITECTURE.md` and `docs/product/PROJECT-MEMORY.md`, plus the known `Research notes` undeclared glossary heading in `docs/product/DOCUMENTATION-MAP.md`.
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts AGENTS.md knowledge evals site/src || true` - PASS, no absolute repository path leaks reported.

`last_validated_commit` remains `null` because this validation was run against the working tree before a commit was created.
