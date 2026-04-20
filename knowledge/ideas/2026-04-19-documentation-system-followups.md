---
id: idea_2026_04_19_documentation_system_followups
kind: idea
status: draft
authority: speculative
created_at: 2026-04-19
updated_at: 2026-04-19
related_docs:
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - evals/specs/2026-04-19-doc-consistency-evals.md
---

# Documentation system follow-up ideas

## Purpose

Save documentation-system ideas without promoting them into current architecture prematurely.

## Ideas

| Idea | Status | Recommendation | Trigger | Risk |
|---|---|---|---|---|
| Documentation map | active-core process | adopt | now | low |
| Shared glossary | active-core process | adopt | now | low |
| Docs consistency eval | active-core eval later | adopt/defer | after eval smoke harness shape stabilizes | medium |
| Duplicate definition detector | incubating | defer | after glossary is accepted | medium |
| Link integrity checker | incubating | defer | after docs map is accepted | low |
| ADR/product-doc drift checker | incubating | defer | after several ADRs touch the same surface | medium |
| Roadmap/status consistency checker | incubating | defer | after crate status changes repeatedly | medium |
| Generated current-state page | future | defer | after inspect/views are stable | medium |
| Public docs website generation | future/public | park | after public narrative and PubPunk scope promotion | medium |
| LLM semantic doc judge as hard gate | future | avoid for now | reconsider after deterministic checks exist | high |

## Anti-patterns

| Anti-pattern | Why dangerous | Mitigation |
|---|---|---|
| One giant product manifesto | Hides owners and makes updates risky | Documentation map + source-of-truth matrix |
| Duplicated schemas in multiple docs | Creates silent drift | Single schema owner + links |
| Roadmap as idea backlog | Promotes future scope too early | Keep ideas in `knowledge/ideas/` |
| ADR as current operator docs | Rationale becomes stale | Product docs reflect accepted decisions |
| Research notes as architecture truth | Bypasses promotion path | Research Gate promotion rule |
| Public narrative overclaiming readiness | Misleads users | Public claim refs and receipts |

