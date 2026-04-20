# Review-window cases

Fixture-style cases for the advisory `review_after` freshness check on touched canonical docs.

Scope of this slice:

- warning-level only;
- touched/new canonical docs only;
- deterministic date handling via `--today YYYY-MM-DD`;
- expected results are `pass`, with warnings only when `review_after` is in the past.

Check family name used in these fixtures:

- `docs_stale_review_window`
