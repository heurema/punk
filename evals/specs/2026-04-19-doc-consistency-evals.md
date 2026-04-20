# Eval spec: documentation consistency

Status: proposed eval spec  
Last updated: 2026-04-19

## Purpose

Punk documentation is now part of the trust surface. This eval spec defines lightweight checks to prevent docs from drifting, duplicating ownership, or presenting parked/future capabilities as active.

## Scope

This spec checks documentation consistency only. It does not decide product acceptance. Eval results may inform `gate`, but only `gate` writes final decisions.

## Inputs

- `README.md`
- `docs/product/*.md`
- `docs/adr/*.md`
- `docs/adapters/*.md`
- `docs/modules/*.md`
- `knowledge/research/*.md`
- `knowledge/ideas/*.md`
- `work/goals/*.md`
- `work/reports/*.md`
- `publishing/**/*.md`

## Hard-gate cases

| Case id | Check | Failure condition |
|---|---|---|
| `docs_no_production_ready_claim` | Public/current docs must not claim Punk is production-ready | Any current doc says or implies production-ready/turnkey without explicit negation |
| `docs_no_future_as_active` | Parked/future capabilities must not appear as current operator path | MCP, adapters, marketplace, autonomous agents, PubPunk automation, plugin runtime, cloud sync, or dashboards appear as active/default |
| `docs_gate_decision_exclusive` | Only `gate` writes final decisions | Any doc says module, adapter, eval, telemetry, policy, or research accepts/decides final work |
| `docs_research_not_truth` | Research stays advisory until promotion | Research note used as direct implementation truth without ADR/roadmap/contract path |
| `docs_idea_not_truth` | Idea backlog stays speculative | `knowledge/ideas/` item appears as active architecture without promotion refs |
| `docs_no_hidden_memory_truth` | Hidden runtime/retrieval memory is not project truth | Vector/index/cache/history memory described as canonical project memory |
| `docs_no_remote_export_default` | Remote export is parked and opt-in later | Telemetry/export/cloud sync described as default current behavior |
| `docs_schema_single_owner` | Schemas have one owner | Same schema definition copied in multiple docs with divergent fields |

## Warning cases

| Case id | Check | Warning condition |
|---|---|---|
| `docs_reading_order_present` | `START-HERE.md` links to required canonical docs | Missing docs from current reading order |
| `docs_source_owner_present` | `DOCUMENTATION-MAP.md` covers all product docs | Product doc exists without owner row |
| `docs_status_vocabulary` | Status terms use exact vocabulary | Unknown status terms appear without definition |
| `docs_adrs_link_research` | Accepted ADRs link research when required | ADR lacks research ref for architecture-affecting decision |
| `docs_roadmap_links_evals` | Roadmap phases mention eval direction | Phase lacks eval implication |
| `docs_public_claim_refs` | Public claims cite canonical docs or mark opinion | Public narrative makes unsupported product claims |
| `docs_glossary_linkage` | Reused terms link to glossary | Repeated local definitions drift from glossary |

## Suggested output

Machine JSON:

```json
{
  "schema_version": "punk.eval.docs_consistency.v0.1",
  "suite_id": "docs_consistency",
  "status": "pass|fail|warn",
  "cases": [
    {
      "case_id": "docs_no_future_as_active",
      "status": "pass|fail|warn",
      "severity": "hard|warning",
      "files": ["docs/product/START-HERE.md"],
      "message": "parked capability described as active",
      "artifact_refs": []
    }
  ]
}
```

Human Markdown:

```markdown
# Docs consistency eval report

Status: pass|fail|warn

## Hard gates

- docs_no_future_as_active: pass

## Warnings

- docs_glossary_linkage: warn — term repeated in two docs
```

## Implementation notes

Start with deterministic text fixtures and allowlist/denylist checks. Do not add LLM judging as the first implementation.

Possible later checks:

- link integrity
- frontmatter validation
- ADR status validation
- duplicate section detection
- glossary drift detection
- phase/status consistency between `ROADMAP.md` and `CRATE-STATUS.md`

## Out of scope

- semantic correctness judging by LLM
- external link crawling as a hard gate
- public website generation
- remote documentation analytics

