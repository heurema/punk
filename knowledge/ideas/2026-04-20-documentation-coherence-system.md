---
id: idea_2026_04_20_documentation_coherence_system
title: "Documentation coherence system"
kind: idea
status: raw
authority: speculative
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
review_after: 2026-07-20
components: [documentation, project-memory, evals, contract-tracker]
related_files:
  - docs/product/DOC-GOVERNANCE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - evals/specs/docs-consistency.v0.1.md
source_refs:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-INTAKE.md
confidence: high
---

# Documentation coherence system

These ideas shape how documentation becomes a first-class lifecycle artifact inside Punk.

| Idea | Status | Why preserve | Promotion condition |
|---|---|---|---|
| `DocImpact` on every meaningful change | adopt-now process | Prevents silent doc drift after code or architecture changes. | Make it a required contract field in Phase 3. |
| Standalone `DocImpact` schema reused by reports and contracts | adopt-now process | Keeps one shape across manual and automated phases. | Contract schema lands without a second format. |
| Frontmatter on new/touched canonical docs | adopt-now process | Gives status, authority, owner, and review windows to docs truth. | Migrate legacy canonical docs gradually. |
| Documentation owner registry | adopt-now process | Stops multiple files from owning the same truth. | Keep `DOCUMENTATION-MAP.md` current. |
| Glossary-backed term lint | adopt-now warning | Reduces wording drift around `gate`, `assessment`, `proofpack`, and status terms. | Add deterministic term checks. |
| Docs consistency eval family | adopt-now eval direction | Makes docs integrity inspectable without letting docs checks decide. | Add deterministic smoke checks in Phase 2. |
| No silent deletion of current truth | adopt-now rule | Historical truth must remain inspectable when replaced. | Enforce supersede/archive/retire checks. |
| ADR supersession discipline | adopt-now process | Decision history should survive architecture changes. | Add supersession checks and metadata. |
| `punk docs check` public CLI | defer | Useful later, but current active surface should stay small. | Promote only after eval runner and contract loop stabilize. |
| Automatic doc rewriting | park | Too easy to mutate truth without an explicit decision. | Revisit only as advisory tooling. |
| External wiki as canonical source | avoid | Breaks repo-tracked project memory and owner discipline. | Do not promote. |
