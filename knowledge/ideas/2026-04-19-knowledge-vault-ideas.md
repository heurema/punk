---
id: idea_2026_04_19_knowledge_vault_backlog
title: "Knowledge Vault backlog ideas"
kind: idea
status: raw
authority: speculative
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [project-memory, knowledge-vault]
related_goals:
  - goal_research_knowledge_vault
related_files:
  - knowledge/research/2026-04-19-knowledge-vault-prior-art-summary.md
  - docs/adr/ADR-0008-knowledge-vault-boundaries.md
source_refs:
  - knowledge/research/2026-04-19-knowledge-vault-prior-art-summary.md
confidence: medium
---

# Knowledge Vault backlog ideas

These ideas are worth preserving, but they are not implementation truth and must stay out of default implementation retrieval until promoted.

| Idea | Status | Why preserve | Promotion condition |
|---|---|---|---|
| Tool gap signals | parked | Repeated workaround units may reveal missing tools or weak integrations. | Phase 5 research plus evals. |
| Nomination-not-move | adopt-later | Promotion should not delete or mutate the source artifact. | ADR acceptance plus artifact pattern. |
| Contradiction-first retrieval | incubating | Retrieval should surface conflict instead of hiding it. | Contradiction fixture plus review UX. |
| Proof-linked knowledge promotion | adopt-now as constraint | Canonical knowledge should cite what proved it. | ADR plus eval coverage. |
| Temporal validity windows | parked | Knowledge should represent when it was true and when it became stale. | Later storage research. |
| Knowledge tombstones | parked | Superseded knowledge should retain provenance without affecting defaults. | Stale/superseded evals. |
| Git-backed knowledge index | incubating | Deterministic index over markdown fits local-first repo truth. | Prototype after Phase 4 research. |
| SQLite FTS derived index | incubating | Fast deterministic retrieval may be useful without embeddings. | No-hidden-truth eval. |
| Local-only vector index | parked | Better recall is tempting, but inspectability risk is high. | Must remain derived and disposable. |
| Sigstore-like transparency for proofpacks | future | Verifiable proofpack history could strengthen trust in public builds. | Later provenance research. |
| SLSA/in-toto-style proof attestations | future | Contract/run/proof could gain stronger provenance framing. | Proofpack ADR. |
| Manual receipt-to-knowledge candidate extractor | incubating | Session reports could suggest candidate knowledge without auto-promoting it. | Must stay review-only. |
| Model-family diversity confirmation | parked | Cross-model confirmation may reduce monoculture bias. | Separate evidence model. |
| Commons governance | future | Shared knowledge may matter later, but only after local stability. | Separate governance research. |
| Graphiti adapter | future | Temporal memory views may become useful later. | Adapter-phase research. |
| OpenMemory or mem0 adapter | future | Could bridge project memory across tools later. | Adapter-boundary research. |
| Knowledge lint | incubating | Missing authority/status/proof refs should become detectable. | Phase 4 eval/lint work. |
| Poisoned memory quarantine | adopt-later eval | Prompt injection and secret leakage must be tested before retrieval promotion. | Required security fixture. |
