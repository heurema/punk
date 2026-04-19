# Knowledge

Repo-tracked knowledge vault.

Canonical truth and advisory knowledge are separated by:

- folder
- `kind`
- `status`
- `authority`

Implementation retrieval excludes `authority: speculative` by default.

## Folders

- `knowledge/research/` - curated research notes; advisory by default until promoted by ADR, roadmap, or contract.
- `knowledge/ideas/` - raw or draft ideas; speculative by default and never implementation truth by themselves.
- `knowledge/_templates/` - templates for new knowledge artifacts.
- `knowledge/_schema/` - schema for knowledge artifact metadata.
