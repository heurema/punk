# Bounded diff checklist

Before editing:

- confirm `selected_next` or explicit user override;
- read the selected goal scope and exclusions;
- name the single logical diff you are about to make;
- list the required checks for that diff.

Before staging:

- verify no unrelated cleanup leaked into the diff;
- verify repo-tracked refs are repo-relative;
- verify `.punk/` and other out-of-scope surfaces were not touched;
- verify the selected goal, `work/STATUS.md`, and a report were updated for meaningful work.

Before finishing:

- run `python3 scripts/check_work_ledger.py`;
- run `cargo check --workspace` if Rust changed;
- run docs governance checks if docs, reports, or work-ledger artifacts changed;
- state what remains out of scope;
- make the next selected goal explicit.
