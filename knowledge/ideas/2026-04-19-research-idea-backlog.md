---
id: idea_2026_04_19_research_idea_backlog
title: "Research idea backlog for punk"
kind: idea
status: draft
authority: speculative
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
related_research: knowledge/research/2026-04-19-project-ideas-intake.md
related_docs:
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/TELEMETRY.md
---

# Research idea backlog

Date: 2026-04-19
Status: idea vault
Source: project mechanism research intake

Scoring scale: 1 low, 5 high.

- Core fit: strengthens active `punk` core
- Trust impact: improves proof, gate, auditability, or inspectability
- Local-first: compatible with local-first defaults
- Cost: implementation cost; 1 cheap, 5 expensive
- Prematurity: risk of doing now; 1 low, 5 high
- Differentiation: helps `punk` become distinct

## Top ideas to keep visible

| Rank | Idea | Status | Recommendation | Core | Trust | Local | Cost | Prematurity | Differentiation |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|
| 1 | Append-only local event ledger | active-core | adopt | 5 | 5 | 5 | 2 | 1 | 4 |
| 2 | Replay-derived flow inspect | active-core | adopt | 5 | 5 | 5 | 3 | 1 | 5 |
| 3 | Minimal proofpack manifest | active-core | adopt | 5 | 5 | 5 | 3 | 1 | 5 |
| 4 | Guard denial receipts with reason codes | active-core | adopt | 5 | 5 | 5 | 2 | 1 | 4 |
| 5 | No-network default eval | active-core | adopt | 5 | 5 | 5 | 2 | 1 | 4 |
| 6 | Redaction fixture suite | active-core | adopt | 5 | 5 | 5 | 3 | 1 | 4 |
| 7 | Machine JSON + human Markdown eval report | active-core | adopt | 5 | 4 | 5 | 2 | 1 | 3 |
| 8 | Project-memory link graph | active-core | adopt | 5 | 4 | 5 | 3 | 2 | 5 |
| 9 | Event root/hash chain | incubating | defer | 4 | 5 | 5 | 3 | 2 | 4 |
| 10 | Sandbox grant receipts | incubating | defer | 4 | 5 | 5 | 4 | 2 | 5 |
| 11 | Derived inspect views under `.punk/views/` | active-core later | adopt/defer | 4 | 4 | 5 | 3 | 2 | 4 |
| 12 | Stale memory detector | active-core later | defer | 4 | 4 | 5 | 3 | 2 | 5 |
| 13 | Research Gate card template | active-core | adopt | 4 | 4 | 5 | 1 | 1 | 3 |
| 14 | Decision diff view | incubating | defer | 4 | 4 | 5 | 3 | 2 | 4 |
| 15 | Public build digest from accepted work | future/public | defer | 3 | 4 | 5 | 2 | 3 | 4 |

## Full idea backlog

| Idea | Source pattern | Punk mapping | Status | Risk | Recommendation | Required docs/evals | Core | Trust | Local | Cost | Prematurity | Diff |
|---|---|---|---|---|---|---|---:|---:|---:|---:|---:|---:|
| Append-only local event ledger | Jujutsu op log, event sourcing | `.punk/events/*.jsonl` | active-core | schema churn | adopt | `TELEMETRY.md`, replay eval | 5 | 5 | 5 | 2 | 1 | 4 |
| Replay-derived flow inspect | Jujutsu `--at-op`, event replay | inspect state from event log | active-core | replay bugs | adopt | inspect eval | 5 | 5 | 5 | 3 | 1 | 5 |
| Minimal proofpack manifest | SLSA provenance | manifest links evidence hashes | active-core | over-modeling | adopt | proofpack ADR/eval | 5 | 5 | 5 | 3 | 1 | 5 |
| Guard denial receipts | OPA decision logs | guard result events | active-core | verbose UX | adopt | guard eval | 5 | 5 | 5 | 2 | 1 | 4 |
| Rule explanation strings + codes | policy-as-code | human + machine reason | active-core | inconsistency | adopt | rule fixture | 5 | 5 | 5 | 2 | 1 | 4 |
| No-network default eval | Deno permissions | hard check for no hidden export | active-core | false positives | adopt | privacy eval | 5 | 5 | 5 | 2 | 1 | 4 |
| Redaction fixture suite | OPA masking, telemetry privacy | reject raw sensitive fields | active-core | accidental leaks | adopt | redaction eval | 5 | 5 | 5 | 3 | 1 | 4 |
| Machine JSON eval report | local eval harnesses | stable machine output | active-core | schema versioning | adopt | eval spec | 5 | 4 | 5 | 2 | 1 | 3 |
| Human Markdown eval report | human-readable evidence | readable report for dogfooding | active-core | duplicate truth | adopt | eval report spec | 4 | 4 | 5 | 2 | 1 | 3 |
| Event root/hash chain | transparency-log ideas | tamper-evident local chain | incubating | complexity | defer | ADR + tamper eval | 4 | 5 | 5 | 3 | 2 | 4 |
| Artifact hash normalizer | provenance discipline | stable hash refs | active-core | path normalization | adopt | hash fixture | 5 | 5 | 5 | 3 | 1 | 4 |
| Derived inspect views | view-model patterns | `.punk/views/` generated | active-core later | second-truth risk | adopt/defer | view consistency eval | 4 | 4 | 5 | 3 | 2 | 4 |
| Project-memory link graph | ADR/RFC systems | goal -> contract -> report -> decision -> proof | active-core | link rot | adopt | memory-link eval | 5 | 4 | 5 | 3 | 2 | 5 |
| Stale memory detector | knowledge systems | detect docs not updated after accepted work | active-core later | noisy warnings | defer | memory drift eval | 4 | 4 | 5 | 3 | 2 | 5 |
| Research Gate card template | RFC/ADR systems | standard intake card | active-core | process overhead | adopt | docs lint | 4 | 4 | 5 | 1 | 1 | 3 |
| Sandbox grant receipt | Deno/WASI permissions | explicit file/net/run grant evidence | incubating | platform complexity | defer | sandbox eval | 4 | 5 | 5 | 4 | 2 | 5 |
| Permission scope DSL | Deno permissions | `read:work/**`, `net:none` | incubating | DSL trap | defer | permissions ADR | 4 | 5 | 5 | 4 | 3 | 5 |
| Dry-run side-effect plan | planning-before-apply | show intended writes before run | incubating | incomplete plan | defer | dry-run eval | 4 | 5 | 5 | 4 | 3 | 5 |
| Decision diff view | VCS diff history | compare candidate vs final decision | incubating | UI scope | defer | inspect eval | 4 | 4 | 5 | 3 | 2 | 4 |
| Contract fixture registry | test harnesses | reusable lifecycle fixtures | active-core | maintenance | adopt | fixture spec | 5 | 4 | 5 | 3 | 1 | 3 |
| Failure code taxonomy | test/report patterns | stable errors for evals | active-core | bikeshedding | adopt | error-code doc | 4 | 4 | 5 | 2 | 1 | 3 |
| Local project identity file | local-first roots | stable local id under `.punk/` | active-core | privacy tradeoff | adopt | identity ADR | 5 | 4 | 5 | 2 | 1 | 4 |
| Public build digest | changelog/release notes | public summary from accepted proofs | future | premature narrative | defer | public narrative doc | 3 | 4 | 5 | 2 | 3 | 4 |
| Event replay visualizer | VCS graph | future inspect graph | future | UI scope | park | none yet | 3 | 3 | 5 | 4 | 4 | 4 |
| Remote transparency log adapter | Rekor-like systems | publish proof digest | future/parked | external dependency | park | R2 + ADR | 2 | 4 | 2 | 4 | 5 | 3 |
| OTel export adapter | OpenTelemetry | export sanitized traces | future/parked | privacy/network | park | R2 + redaction eval | 2 | 3 | 2 | 4 | 5 | 2 |
| Action graph | DAG engines | contract steps as DAG | future | complexity | defer | DAG research | 3 | 3 | 4 | 5 | 4 | 3 |
| Action cache | build systems | skip unchanged work | future | invalid proof if wrong | defer | cache ADR/evals | 3 | 4 | 4 | 5 | 4 | 3 |
| Hermetic runner | reproducible runners | reproducible command sandbox | incubating | platform cost | defer | sandbox research | 4 | 5 | 4 | 5 | 3 | 5 |
| Rich policy language | OPA/CUE | configurable guards | future | policy owns truth | defer | policy ADR | 3 | 4 | 5 | 4 | 4 | 3 |
| Schema validation for contracts and proofs | schema systems | contract/proof schema checks | active-core | schema churn | adopt | schema fixture | 4 | 4 | 5 | 2 | 1 | 3 |
| Agent patch proposal mode | coding-agent workflows | agent drafts, human/gate decides | future | agent hype | park | agent evals | 2 | 3 | 4 | 5 | 5 | 3 |
| Agent permission manifest | capability systems | tool grants for agents | future | security | park | sandbox + eval first | 2 | 4 | 4 | 5 | 5 | 4 |
| Local-only dashboard | observability tooling | read from `.punk/views/` | future | UI distraction | defer | view consistency | 2 | 3 | 5 | 4 | 4 | 3 |
| Workspace snapshots | working-copy snapshots | snapshot dirty state before run | incubating | privacy/path issues | defer | snapshot ADR | 3 | 4 | 5 | 4 | 3 | 4 |
| Contract timeline | lifecycle timeline | timeline from events | active-core later | derived-truth risk | defer | replay eval | 4 | 4 | 5 | 3 | 2 | 4 |
| Anti-pattern registry | postmortems | explicit avoid list | active-core | maintenance | adopt | docs lint | 4 | 3 | 5 | 1 | 1 | 3 |
| Module permission matrix | capability systems | assess vs decide vs invoke | active-core | too detailed early | adopt/defer | permissions doc | 4 | 5 | 5 | 3 | 2 | 4 |
| Research source cards | literature review | source-backed idea cards | active-core | paperwork | adopt | research doc | 4 | 4 | 5 | 2 | 1 | 3 |

## Anti-pattern backlog

| Anti-pattern | Why dangerous | Punk mitigation |
|---|---|---|
| Hidden network telemetry | violates local-first trust | no-network default eval |
| AI writes final decision | breaks gate law | only `gate` writes decision |
| Module owns truth | fragments source of truth | modules assess only |
| Adapter owns truth | external systems become canonical | adapters invoke only |
| Raw prompt telemetry | privacy leak | redaction layer |
| Raw source telemetry | IP/security leak | artifact refs + hashes only |
| Dashboard without local truth | opaque state | derived views from canonical artifacts only |
| Memory as giant prompt | drift and unverifiable recall | linked project-memory artifacts |
| Unbounded agent execution | side effects without proof | future sandbox grants + evals |
| Marketplace before core trust | incentives distort core | park marketplace |
| Full DAG scheduler too early | lifecycle gets swallowed by scheduler | defer DAG |
| Cloud sync too early | external source of truth | park sync |
| Remote transparency log too early | external dependency before local proof | local proofpack first |
| Opaque evals | impossible to trust phase promotion | fixtures + reports |
| Eval without deterministic fixture | non-reproducible acceptance | deterministic-first eval suite |
| Proof without hashes | unverifiable acceptance | proofpack hash refs |
| Policy engine decides | violates gate boundary | assessment artifact only |
