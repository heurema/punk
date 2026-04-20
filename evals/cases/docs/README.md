# Docs eval cases

Fixture-style documentation consistency cases.

Scope of this slice:

- deterministic pass/fail fixtures only;
- no public CLI;
- no runner implementation;
- no mass migration of legacy docs.

These cases intentionally use a lighter fixture format:

```yaml
schema_version: punk.docs_eval_case.v0.1
case_id: docs.example.pass
kind: docs-consistency-case
status: active
authority: canonical

description: "Short description."

input:
  changed_files: []
  files: {}

expected:
  result: pass|fail
  hard_failures: []
  warnings: []

checks:
  - docs_frontmatter_schema
```

They are meant to exercise the checks named in `evals/specs/docs-consistency.v0.1.md` until the deterministic runner lands.
